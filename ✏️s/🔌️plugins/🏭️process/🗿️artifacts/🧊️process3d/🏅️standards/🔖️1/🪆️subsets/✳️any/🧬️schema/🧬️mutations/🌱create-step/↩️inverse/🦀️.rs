//! ↩️ `create-step` inverse — undo of a create is always a `delete-step` by the created id.

use crate::delete_step::DeleteStep;
use semio_s_artifact_process_process3d_model::snapshot::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::delete_step::DeleteStep>>(payload: &super::CreateStep, _base: &Process3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Op::from(DeleteStep { id: payload.step.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
