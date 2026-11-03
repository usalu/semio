//! ↩️ `change-reliability-class` — undo restores BASE's reliability class.

use crate::change_reliability_class::ChangeReliabilityClass;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &ChangeReliabilityClass, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_reliability_class::ChangeReliabilityClass> {
    Ok((|| {
    vec![Op::from(ChangeReliabilityClass { new_reliability_class: base.reliability_class })]

    })())
}
//#endregion 🔖️Inverse
