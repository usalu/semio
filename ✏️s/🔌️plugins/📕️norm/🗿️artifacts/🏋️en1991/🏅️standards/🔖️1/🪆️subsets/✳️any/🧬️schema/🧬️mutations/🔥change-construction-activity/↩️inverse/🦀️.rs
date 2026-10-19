//! ↩️ Inverse for `change-construction-activity`.
use crate::change_construction_activity::ChangeConstructionActivity;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeConstructionActivity, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_construction_activity::ChangeConstructionActivity> {
    Ok((|| {
    vec![Op::from(ChangeConstructionActivity { new_construction_activity: base.construction_activity.clone() })]

    })())
}
