//! ↩️ `change-step-origin` inverse — reconstructs the pre-change origin from BASE state; a step
//! already absent from `base` has nothing to undo.

use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ChangeStepOrigin, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
    base.step_payloads.iter().find(|step| step.id == payload.id).map(|step| vec![Process3dMutation::ChangeStepOrigin(super::ChangeStepOrigin { id: payload.id.clone(), new_origin: step.origin.clone() })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
