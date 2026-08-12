//! ↩️ `delete-step` inverse — reconstructs a `create-step` from BASE state (original list
//! position + full payload); a step already absent from `base` has nothing to undo.

use crate::artifacts::process3d::mutations::delete_step::mutation::DeleteStep;
use crate::artifacts::process3d::mutations::create_step::mutation::CreateStep;
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteStep, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
    base.steps
        .iter()
        .position(|step| step.id == payload.id)
        .map(|index| vec![Process3dMutation::CreateStep(CreateStep { index, step: base.steps[index].clone() })])
        .unwrap_or_default()
}
//#endregion 🔖️Inverse
