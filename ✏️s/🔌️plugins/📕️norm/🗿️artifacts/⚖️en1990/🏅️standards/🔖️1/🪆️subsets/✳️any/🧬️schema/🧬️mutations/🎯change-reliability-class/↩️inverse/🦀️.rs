//! ↩️ `change-reliability-class` inverse.

use crate::change_reliability_class::ChangeReliabilityClass;

use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(mutation: &ChangeReliabilityClass, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_reliability_class::ChangeReliabilityClass> {
    Ok((|| {
    let _ = mutation;
    vec![Op::from(ChangeReliabilityClass { new_reliability_class: base.reliability_class })]

    })())
}
