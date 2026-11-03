//! ↩️ `change-reliability-class` inverse.

use crate::change_reliability_class::ChangeReliabilityClass;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(_payload: &ChangeReliabilityClass, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_reliability_class::ChangeReliabilityClass> {
    Ok((|| {
    vec![Op::from(ChangeReliabilityClass { new_reliability_class: base.reliability_class })]

    })())
}
