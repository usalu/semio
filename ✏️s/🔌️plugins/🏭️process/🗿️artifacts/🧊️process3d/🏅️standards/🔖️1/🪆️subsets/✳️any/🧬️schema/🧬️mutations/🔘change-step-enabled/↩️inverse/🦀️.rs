//! ↩️ `change-step-enabled` inverse — reconstructs the pre-change flag from BASE state; a step
//! already absent from `base` has nothing to undo.

use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ChangeStepEnabled, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
    base.step_payloads.iter().find(|step| step.id == payload.id).map(|step| vec![Process3dMutation::ChangeStepEnabled(super::ChangeStepEnabled { id: payload.id.clone(), new_enabled: step.enabled })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
