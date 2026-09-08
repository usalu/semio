//! ↩️ `delete-step` inverse — reconstructs a `create-step` from BASE state (original list position
//! + full payload); a step already absent from `base` has nothing to undo.

use crate::mutations::create_step::CreateStep;
use crate::mutations::Process3dMutation;
use crate::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DeleteStep, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
    base.step_payloads.iter().position(|step| step.id == payload.id).map(|index| vec![Process3dMutation::CreateStep(CreateStep { index, step: base.step_payloads[index].clone() })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
