//! ↩️ `create-step` inverse — undo of a create is always a `delete-step` by the created id.

use crate::mutations::delete_step::DeleteStep;
use crate::mutations::Process3dMutation;
use crate::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::CreateStep, _base: &Process3dSnapshot) -> Result<Vec<Process3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Process3dMutation::DeleteStep(DeleteStep { id: payload.step.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
