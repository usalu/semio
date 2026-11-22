//! 🚰️ Field formats in the April 2026 corrections to VDI 3805 Blatt 29.

use super::CorrectedFields;

/// 📏️ Verify the published positions, preserving other sheet rules for the full specification.
pub(super) fn corrected_fields(family: &str, fields: &[String], date: u32) -> Option<CorrectedFields> {
    if date < 202604 {
        return None;
    }
    let (check_id, correction, numeric, text): (_, _, &[usize], &[usize]) = match family {
        "700" => ("vdi3805.29.record.700", "29-0005", &[13], &[14]),
        "710.01" => ("vdi3805.29.record.710.01", "29-0008", &[12, 14, 15, 16], &[]),
        "710.02" => ("vdi3805.29.record.710.02", "29-0006", &[7, 9, 20], &[]),
        _ => return None,
    };
    let field = |position: usize| fields.get(position - 1).map(|value| value.trim());
    let index = field(2).and_then(|value| value.parse::<i32>().ok());
    let standard = [(1, field(1) == Some(family)), (2, index.is_some_and(|value| (1..=99_999).contains(&value)))];
    let mut result = CorrectedFields { check_id, correction, positions: Vec::new(), invalid: Vec::new() };
    for (position, valid) in standard.into_iter()
        .chain(numeric.iter().map(|&position| (position,
            field(position).is_some_and(|value| value.is_empty() || value.parse::<f64>().is_ok_and(f64::is_finite)))))
        .chain(text.iter().map(|&position| (position, field(position).is_some())))
    {
        result.positions.push(position);
        if !valid {
            result.invalid.push(position);
        }
    }
    Some(result)
}
