//! ⚖️ Published Part 22 corrections for affected fields of existing record families.

#[derive(Debug, PartialEq, Eq)]
pub(super) struct PublishedFields {
    pub check: &'static str,
    pub clause: &'static str,
    pub corrections: &'static str,
    pub positions: Vec<usize>,
    pub invalid: Vec<usize>,
}

fn at(fields: &[String], number: usize) -> Option<&str> {
    fields.get(number - 1).map(|field| field.trim())
}

fn integer(fields: &[String], number: usize) -> Option<i32> {
    at(fields, number)?.parse().ok()
}

fn optional_number(field: Option<&str>) -> bool {
    field.is_some_and(|field| field.is_empty() || field.parse::<f64>().is_ok_and(f64::is_finite))
}

fn optional_value(field: Option<&str>, values: &[&str]) -> bool {
    field.is_some_and(|field| field.is_empty() || values.contains(&field))
}

fn optional_code(field: Option<&str>, values: &[i32]) -> bool {
    field.is_some_and(|field| field.is_empty() || field.parse::<i32>().is_ok_and(|value| values.contains(&value)))
}

fn family_and_index(family: &str, fields: &[String], limit: i32) -> [(usize, bool); 2] {
    [(1, at(fields, 1) == Some(family)), (2, integer(fields, 2).is_some_and(|index| (1..=limit).contains(&index)))]
}

fn technical_data(fields: &[String], date: u32) -> Vec<(usize, bool)> {
    let mut checks = family_and_index("700", fields, 99_999).to_vec();
    if let Some(cooling) = at(fields, 21).filter(|field| !field.is_empty()) {
        checks.push((21, optional_number(Some(cooling)) && integer(fields, 20).is_some_and(|mode| mode != 0)));
    }
    if date >= 202201 {
        let compressor = integer(fields, 40);
        checks.extend([
            (40, optional_code(at(fields, 40), &[1, 2, 3, 4, 5, 99])),
            (41, at(fields, 41).is_some_and(|description| compressor != Some(99) || !description.is_empty())),
        ]);
    }
    let current = date >= 202601;
    let safety_classes: &[&str] = if current {
        &["A1", "A2", "A2L", "A3", "B1", "B2", "B3"]
    } else {
        &["A1", "A2", "A3", "B1", "B2", "B3"]
    };
    checks.extend([
        (34, optional_number(at(fields, 34))),
        (35, optional_number(at(fields, 35))),
        (36, optional_code(at(fields, 36), &[1, 2])),
        (37, optional_value(at(fields, 37), safety_classes)),
        (38, at(fields, 38).is_some()),
        (39, optional_code(at(fields, 39), &[0, 1])),
    ].into_iter().filter(|(position, _)| current || at(fields, *position).is_some()));
    if current {
        checks.extend([
            (42, optional_code(at(fields, 42), &[0, 1, 2, 3, 4, 5, 6])),
            (43, optional_number(at(fields, 43))),
        ]);
    } else {
        checks.extend((42..=43).filter_map(|position| at(fields, position).map(|field| (position, field.is_empty()))));
    }
    checks
}

/// 🧾️ Verifies only the positions published in each applicable correction.
pub(super) fn published_fields(family: &str, fields: &[String], date: u32) -> Option<PublishedFields> {
    let (check, clause, corrections, mut checks): (_, _, _, Vec<_>) = match family {
        "110" => {
            let last_public_type = if date >= 202601 { 7 } else { 6 };
            let valid = integer(fields, 2).is_some_and(|code| (1..=last_public_type).contains(&code) || (100..=999).contains(&code));
            ("vdi3805.22.record110", "5.2.1", "22-0010", vec![(1, at(fields, 1) == Some(family)), (2, valid)])
        }
        "400" => {
            let maximum = if date >= 202601 { 4 } else { 2 };
            let valid = integer(fields, 2).is_some_and(|code| (1..=maximum).contains(&code));
            ("vdi3805.22.record400", "5.2.1", "22-0011", vec![(1, at(fields, 1) == Some(family)), (2, valid)])
        }
        "710.09" => {
            let mut checks = family_and_index(family, fields, i32::MAX).to_vec();
            checks.extend([
                (3, matches!(integer(fields, 3), Some(1 | 2))),
                (4, optional_number(at(fields, 4))),
                (5, optional_number(at(fields, 5))),
                (6, optional_number(at(fields, 6))),
                (7, at(fields, 7).is_some_and(|field| field.is_empty() || integer(fields, 7).is_some_and(|stage| stage > 0))),
                (8, optional_number(at(fields, 8))),
            ]);
            checks.extend((9..=fields.len()).map(|position| (position, false)));
            ("vdi3805.22.record71009", "5.2.3", "22-0004", checks)
        }
        "710.10" => {
            let mut checks = family_and_index(family, fields, i32::MAX).to_vec();
            let maximum = if date >= 201911 { 10 } else { 9 };
            checks.push((7, integer(fields, 7).is_some_and(|class| (1..=maximum).contains(&class))));
            ("vdi3805.22.record71010", "5.2.3", "22-0001", checks)
        }
        "700" => ("vdi3805.22.record700", "5.2.3", "22-0003 / 22-0013", technical_data(fields, date)),
        _ => return None,
    };
    checks.sort_unstable_by_key(|(position, _)| *position);
    let positions = checks.iter().map(|(position, _)| *position).collect();
    let invalid = checks.into_iter().filter_map(|(position, valid)| (!valid).then_some(position)).collect();
    Some(PublishedFields { check, clause, corrections, positions, invalid })
}
