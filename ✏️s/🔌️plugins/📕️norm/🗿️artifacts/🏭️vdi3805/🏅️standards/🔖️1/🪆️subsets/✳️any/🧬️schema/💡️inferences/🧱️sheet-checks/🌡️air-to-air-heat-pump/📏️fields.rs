//! 🌡️ VDI 3805 Blatt 22 corrections 22-0005 through 22-0008.

/// 🧾️ Invalid field positions use the one-based numbering in the published corrections.
#[derive(Debug, PartialEq, Eq)]
pub(super) enum RecordError {
    FieldCount(usize),
    Fields(Vec<usize>),
}

use Rule::*;

enum Rule {
    Family,
    Index,
    Codes(&'static [i32]),
    DescriptionWhenOther(usize),
    Lot10Class,
    OptionalInteger,
    Number,
}

const HEAT_PUMP: &[Rule] = &[
    Family, Index, Codes(&[1, 2, 99]), DescriptionWhenOther(2),
    Number, Number, Number, Number, Number, Number, Number,
    Codes(&[1, 2, 3, 99]), DescriptionWhenOther(11),
    Codes(&[0, 1]), Codes(&[0, 1]), Number, Number, OptionalInteger, Codes(&[1, 2]),
];
const ERP: &[Rule] = &[
    Family, Index, Codes(&[1, 2]), Codes(&[1, 2]), Lot10Class,
    Number, Number, Number, Number, Number, Number, Number, Number,
    Codes(&[1, 2, 3]), Number,
];
const ERP_INSTALLATION: &[Rule] = &[Family, Index, Codes(&[1, 2]), Number, Number];

/// ✅️ Checks field count, formats, enumerations and conditions for a supported Part 22 record family.
pub(super) fn validate_fields(family: &str, fields: &[String]) -> Option<Result<(), RecordError>> {
    let rules = match family {
        "710.15" => HEAT_PUMP,
        "710.16" => ERP,
        "710.17" => ERP_INSTALLATION,
        _ => return None,
    };
    if fields.len() != rules.len() {
        return Some(Err(RecordError::FieldCount(rules.len())));
    }
    let integer = |index: usize| fields[index].trim().parse::<i32>().ok();
    let invalid: Vec<_> = rules.iter().enumerate().filter_map(|(position, rule)| {
        let value = fields[position].trim();
        let valid = match rule {
            Family => value == family,
            Index => integer(position).is_some_and(|index| index > 0),
            Codes(allowed) => integer(position).is_some_and(|code| allowed.contains(&code)),
            DescriptionWhenOther(code_field) => integer(*code_field) != Some(99) || !value.is_empty(),
            Lot10Class => match integer(2) {
                Some(1) => integer(position).is_some_and(|code| (1..=10).contains(&code)),
                Some(2) => value.is_empty(),
                _ => true,
            },
            OptionalInteger => value.is_empty() || integer(position).is_some(),
            Number => value.is_empty() || value.parse::<f64>().is_ok_and(f64::is_finite),
        };
        (!valid).then_some(position + 1)
    }).collect();
    Some(if invalid.is_empty() { Ok(()) } else { Err(RecordError::Fields(invalid)) })
}
