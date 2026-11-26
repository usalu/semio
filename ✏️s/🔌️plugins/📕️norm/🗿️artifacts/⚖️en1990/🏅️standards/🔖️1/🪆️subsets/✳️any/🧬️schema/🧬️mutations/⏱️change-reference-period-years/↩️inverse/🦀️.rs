//! ↩️ `change-reference-period-years` inverse.

use crate::change_reference_period_years::ChangeReferencePeriodYears;

use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(mutation: &ChangeReferencePeriodYears, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_reference_period_years::ChangeReferencePeriodYears> {
    Ok(match previous(mutation, base) {
        Some(inverse) => vec![Op::from(inverse)],
        None => Vec::new(),
    })
}

fn previous(_mutation: &ChangeReferencePeriodYears, base: &En1990Snapshot) -> Option<crate::change_reference_period_years::ChangeReferencePeriodYears> {
    Some(ChangeReferencePeriodYears { new_reference_period_years: base.reference_period_years })
}
