//! ↩️ `change-reference-period-years` inverse.

use crate::change_reference_period_years::ChangeReferencePeriodYears;

use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(mutation: &ChangeReferencePeriodYears, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_reference_period_years::ChangeReferencePeriodYears> {
    Ok((|| {
    let _ = mutation;
    vec![Op::from(ChangeReferencePeriodYears { new_reference_period_years: base.reference_period_years })]

    })())
}
