//! ↩️ `rename-machine` inverse — reconstructs the pre-rename label from BASE state; a machine
//! already absent from `base` has nothing to undo.

use crate::mutations::Process3dMutation;
use crate::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::RenameMachine, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
    base.workshop.machines.iter().find(|machine| machine.id == payload.id).map(|machine| vec![Process3dMutation::RenameMachine(super::RenameMachine { id: payload.id.clone(), new_label: machine.label.clone() })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
