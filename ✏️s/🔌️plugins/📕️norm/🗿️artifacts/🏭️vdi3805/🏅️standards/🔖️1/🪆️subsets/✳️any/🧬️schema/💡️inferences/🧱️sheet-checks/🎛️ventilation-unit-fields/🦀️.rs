//! 🎛️ VDI 3805 Blatt 23 correction fields for ventilation equipment.

use super::CorrectedFields;

/// 📏️ Check the fields released in corrections 23-0001 through 23-0003.
pub(super) fn corrected_fields(
    family: &str,
    fields: &[String],
    date: u32,
) -> Option<CorrectedFields> {
    let field = |position: usize| fields.get(position - 1).map(|value| value.trim());
    let integer = |position| field(position).and_then(|value| value.parse::<i32>().ok());
    let numeric = |position| field(position).is_some_and(|value| {
        value.is_empty() || value.parse::<f64>().is_ok_and(f64::is_finite)
    });
    let (check_id, correction, maximum_index, checks) = match family {
        "710.03" if date >= 202001 => ("vdi3805.23.record.710.03", "23-0001", 99_999, vec![
            (3, matches!(field(3), Some("AC" | "DC" | "EC"))),
            (4, integer(4).is_some_and(|value| value >= 0)),
        ]),
        "710.17" if date >= 202001 => ("vdi3805.23.record.710.17", "23-0002", 99_999, vec![(5, numeric(5))]),
        "710.11" if date >= 202501 => {
            let mut checks: Vec<_> = (3..=13).map(|position| (position, numeric(position))).collect();
            checks.extend([
                (14, field(14) == Some("")),
                (15, integer(15).is_some_and(|position| (1..=5).contains(&position))),
                (16, numeric(16)),
                (17, numeric(17)),
            ]);
            ("vdi3805.23.record.710.11", "23-0003", i32::MAX, checks)
        }
        _ => return None,
    };
    let base = [
        (1, field(1) == Some(family)),
        (2, integer(2).is_some_and(|index| (1..=maximum_index).contains(&index))),
    ];
    let mut result = CorrectedFields {
        check_id,
        correction,
        positions: Vec::new(),
        invalid: Vec::new(),
    };
    for (position, valid) in base.into_iter().chain(checks) {
        result.positions.push(position);
        if !valid {
            result.invalid.push(position);
        }
    }
    Some(result)
}
