//! ↩️ `rename-step` inverse — reconstructs the pre-rename label from BASE state; a step already
//! absent from `base` has nothing to undo.

use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::RenameStep, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
    base.step_payloads.iter().find(|step| step.id == payload.id).map(|step| vec![Process3dMutation::RenameStep(super::RenameStep { id: payload.id.clone(), new_label: step.label.clone() })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
