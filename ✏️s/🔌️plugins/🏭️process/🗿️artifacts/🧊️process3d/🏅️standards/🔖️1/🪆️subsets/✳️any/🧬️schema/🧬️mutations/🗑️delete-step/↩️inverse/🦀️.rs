//! ↩️ `delete-step` inverse — reconstructs a `create-step` from BASE state (original list position
//! + full payload); a step already absent from `base` has nothing to undo.

use crate::artifacts::process3d::mutations::create_step::CreateStep;
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DeleteStep, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
    base.step_payloads.iter().position(|step| step.id == payload.id).map(|index| vec![Process3dMutation::CreateStep(CreateStep { index, step: base.step_payloads[index].clone() })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
