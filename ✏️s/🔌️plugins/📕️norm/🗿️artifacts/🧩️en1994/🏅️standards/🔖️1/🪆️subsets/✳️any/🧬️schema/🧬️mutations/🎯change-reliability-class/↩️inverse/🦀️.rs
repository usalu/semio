//! ↩️ Inverse for `change-reliability-class`.
use crate::change_reliability_class::ChangeReliabilityClass;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(_payload: &ChangeReliabilityClass, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_reliability_class::ChangeReliabilityClass> {
    Ok((|| {
    vec![Op::from(ChangeReliabilityClass { new_reliability_class: base.reliability_class })]

    })())
}
