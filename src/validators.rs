use std::cmp::PartialEq;
use std::cmp::PartialOrd;
use std::fmt::Display;
use std::clone::Clone;

#[cfg(feature = "inclusive_range")]
use std::ops::RangeInclusive;

/// Enforce that a `String` is minimum `min` characters long.
pub fn min(min: usize) -> Box<Fn(&String) -> ::ValidatorResult> {
    Box::new(move |s: &String| {
        if s.len() >= min {
            Ok(())
        } else {
            Err(::Invalid {
                msg: "Must contain more than %1 characters".to_string(),
				args: vec![min.to_string()],
				human_readable: format!("Must contain more than {} characters", min)
            })
        }
    })
}

/// Enforce that a `String` is maximum `max` characters long.
pub fn max(max: usize) -> Box<Fn(&String) -> ::ValidatorResult> {
    Box::new(move |s: &String| {
        if s.len() <= max {
            Ok(())
        } else {
            Err(::Invalid {
                msg: "Must not contain more characters than %1.".to_string(),
                args: vec![max.to_string()],
				human_readable: format!("Must contain less than {} characters", max)
            })
        }
    })
}

#[cfg(not(feature = "inclusive_range"))]
/// Enforce that a string is minimum `mi` and maximum `ma` characters long.
pub fn length(mi: usize, ma: usize) -> Box<Fn(&String) -> ::ValidatorResult> {
    Box::new(move |s: &String| {
        match (min(mi)(s), max(ma)(s)) {
            (Err(_), Err(_)) => {
                Err(::Invalid {
                    msg: "Must not be less characters than %1 and not more than %2.".to_string(),
                    args: vec![mi.to_string(), ma.to_string()],
					human_readable: format!("Must contain between {} and {} characters", mi, ma - 1)
                })
            }
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
            (_, _) => Ok(()),
        }
    })
}

#[cfg(not(feature = "inclusive_range"))]
/// Enforce that a string is minimum `mi` and maximum `ma` characters long if it is present. Always ok if not present.
pub fn length_if_present(mi: usize, ma: usize) -> Box<Fn(&Option<String>) -> ::ValidatorResult> {
    Box::new(move |s: &Option<String>| {
		if s.is_none() {
			return Ok(());
		}
		let s = s.as_ref().unwrap();
        match (min(mi)(s), max(ma)(s)) {
            (Err(_), Err(_)) => {
                Err(::Invalid {
                    msg: "Must not be less characters than %1 and not more than %2.".to_string(),
                    args: vec![mi.to_string(), ma.to_string()],
					human_readable: format!("Must contain between {} and {} characters", mi, ma - 1)
                })
            }
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
            (_, _) => Ok(()),
        }
    })
}

#[cfg(feature = "inclusive_range")]
/// Enforce that a string is minimum `mi` and maximum `ma` characters long.
pub fn length(range: RangeInclusive<usize>) -> Box<Fn(&String) -> ::ValidatorResult> {
    Box::new(move |s: &String| {
        match range {
            RangeInclusive::NonEmpty { ref start, ref end } => {
                match (min(*start)(s), max(*end)(s)) {
                    (Err(_), Err(_)) => {
                        Err(::Invalid {
                            msg: "Must not be less characters than %1 and not more than %2."
                                .to_string(),
                            args: vec![start.to_string(), end.to_string()],
							human_readable: format!("Must contain between {} and {} characters", mi, ma)
                        })
                    }
                    (Err(e), _) => Err(e),
                    (_, Err(e)) => Err(e),
                    (_, _) => Ok(()),
                }
            }
            _ => panic!("range must be a RangeInclusive::NonEmpty"),
        }
    })
}

#[cfg(feature = "inclusive_range")]
/// Enforce that a string is minimum `mi` and maximum `ma` characters long if it is present. Always ok if not present.
pub fn length_if_present(range: RangeInclusive<usize>) -> Box<Fn(&Option<String>) -> ::ValidatorResult> {
    Box::new(move |s: &Option<String>| {
		if s.is_none() {
			return Ok(());
		}
		let s = s.as_ref().unwrap();
        match range {
            RangeInclusive::NonEmpty { ref start, ref end } => {
                match (min(*start)(s), max(*end)(s)) {
                    (Err(_), Err(_)) => {
                        Err(::Invalid {
                            msg: "Must not be less characters than %1 and not more than %2."
                                .to_string(),
                            args: vec![start.to_string(), end.to_string()],
							human_readable: format!("Must contain between {} and {} characters", start, end)
                        })
                    }
                    (Err(e), _) => Err(e),
                    (_, Err(e)) => Err(e),
                    (_, _) => Ok(()),
                }
            }
            _ => panic!("range must be a RangeInclusive::NonEmpty"),
        }
    })
}


#[cfg(not(feature = "inclusive_range"))]
pub fn range<T: 'static + PartialOrd + Display + Clone>(a: T,
                                                        b: T)
                                                        -> Box<Fn(&T) -> ::ValidatorResult> {
    Box::new(move |s: &T| {
        if *s >= a && *s <= b {
            Ok(())
        } else {
            Err(::Invalid {
                msg: "Must be in the range %1..%2.".to_string(),
                args: vec![a.to_string(), b.to_string()],
				human_readable: format!("Must be between {} and {}", a, b)
            })
        }
    })
}

#[cfg(feature = "inclusive_range")]
pub fn range<T: 'static + PartialOrd + Display + Clone>(range: RangeInclusive<T>)
                                                        -> Box<Fn(&T) -> ::ValidatorResult> {
    Box::new(move |s: &T| {
        match range {
            RangeInclusive::NonEmpty { ref start, ref end } => {
                if *s >= *start && *s <= *end {
                    Ok(())
                } else {
                    Err(::Invalid {
                        msg: "Must be in the range %1..%2.".to_string(),
                        args: vec![start.to_string(), end.to_string()],
						human_readable: format!("Must be between {} and {}", start, end)
                    })
                }
            } 
            _ => panic!("range must be a RangeInclusive::NonEmpty"),
        }
    })
}

/// Enforce that a string must contain `needle`.
pub fn contains(needle: &'static str) -> Box<Fn(&String) -> ::ValidatorResult> {
    Box::new(move |s: &String| {
        if s.contains(needle) {
            Ok(())
        } else {
            Err(::Invalid {
                msg: "Must contain %1.".to_string(),
                args: vec![needle.to_string()],
				human_readable: format!("Must contain '{}'", needle)
            })
        }
    })
}

/// Enforce that a string contains only characters in `accepted`
pub fn contain_only(accepted: &'static [char]) -> Box<Fn(&String) -> ::ValidatorResult> {
    Box::new(move |s: &String| {
        for c in s.chars() {
			if !accepted.contains(&c) {
				return Err(::Invalid {
					msg: "Must not contain %1.".to_string(),
					args: vec![c.to_string()],
					human_readable: format!("Must not contain '{}'", c)
				});
			}
		}
		Ok(())
    })
}

/// Convenience function; 0-9, A-z
pub fn alphanumeric() -> Box<Fn(&String) -> ::ValidatorResult> {
	contain_only(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'])
}

/// Convenience function; Alphanumeric & underscore.
pub fn alphanumeric_dashes() -> Box<Fn(&String) -> ::ValidatorResult> {
	contain_only(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '_', '-'])
}

/// Enforce that a string must not contain `needle`.
pub fn not_contain(needle: &'static str) -> Box<Fn(&String) -> ::ValidatorResult> {
    Box::new(move |s: &String| {
        if !s.contains(needle) {
            Ok(())
        } else {
            Err(::Invalid {
                msg: "Must not contain %1.".to_string(),
                args: vec![needle.to_string()],
				human_readable: format!("Must not contain '{}'", needle)
            })
        }
    })
}

/// Enforce that a string must not contain any of `needles`.
pub fn not_contain_any(needles: &'static [&'static str]) -> Box<Fn(&String) -> ::ValidatorResult> {
    Box::new(move |s: &String| {
        for needle in needles {
			if s.contains(needle) {
				return Err(::Invalid {
					msg: "Must not contain %1.".to_string(),
					args: vec![needle.to_string()],
					human_readable: format!("Must not contain '{}'", needle)
				});
			}
		}
		Ok(())
    })
}

/// Enforce that `T` must equal `value`.
pub fn eq<T: 'static>(value: T) -> Box<Fn(&T) -> ::ValidatorResult>
    where T: PartialEq + Display
{
    Box::new(move |s: &T| {
        if *s == value {
            Ok(())
        } else {
            Err(::Invalid {
                msg: "Does not equal %1.".to_string(),
                args: vec![value.to_string()],
				human_readable: format!("Does not equal '{}'", value)
            })
        }
    })
}

/// Enforce that `T` equals any of the values in `values`.
pub fn either<T: 'static>(values: Vec<T>) -> Box<Fn(&T) -> ::ValidatorResult>
    where T: PartialEq + Display + Clone
{
    Box::new(move |s: &T| {
        let x = values.iter().find(|x| *x == s);
        let r = x.is_some();
        if r == true {
            Ok(())
        } else {
            let list = values.iter()
                .cloned()
                .fold(String::new(), |acc, v| format!("{}, {}", acc, v));
            Err(::Invalid {
                msg: "Must be one of %1.".to_string(),
                args: vec![list.to_string()],
				human_readable: format!("Must be one of {}", list.to_string())
            })
        }
    })
}

#[cfg(test)]
mod tests {
    use validators::{length, range};

    #[test]
    #[cfg(feature = "inclusive_range")]
    fn test_length() {
        assert!(length(3...5)(&"12".to_string()).is_err());
        assert!(length(3...5)(&"123".to_string()).is_ok());
        assert!(length(3...5)(&"1234".to_string()).is_ok());
        assert!(length(3...5)(&"12345".to_string()).is_ok());
        assert!(length(3...5)(&"123456".to_string()).is_err());
    }

    #[test]
    #[cfg(not(feature = "inclusive_range"))]
    fn test_length() {
        assert!(length(3, 5)(&"12".to_string()).is_err());
        assert!(length(3, 5)(&"123".to_string()).is_ok());
        assert!(length(3, 5)(&"1234".to_string()).is_ok());
        assert!(length(3, 5)(&"12345".to_string()).is_ok());
        assert!(length(3, 5)(&"123456".to_string()).is_err());
    }

    #[test]
    #[cfg(feature = "inclusive_range")]
    fn test_range() {
        assert!(range(12...127)(&11).is_err());
        assert!(range(12...127)(&12).is_ok());
        assert!(range(12...127)(&50).is_ok());
        assert!(range(12...127)(&127).is_ok());
        assert!(range(12...127)(&128).is_err());
    }

    #[test]
    #[cfg(not(feature = "inclusive_range"))]
    fn test_range() {
        assert!(range(12, 127)(&11).is_err());
        assert!(range(12, 127)(&12).is_ok());
        assert!(range(12, 127)(&50).is_ok());
        assert!(range(12, 127)(&127).is_ok());
        assert!(range(12, 127)(&128).is_err());
    }
}
