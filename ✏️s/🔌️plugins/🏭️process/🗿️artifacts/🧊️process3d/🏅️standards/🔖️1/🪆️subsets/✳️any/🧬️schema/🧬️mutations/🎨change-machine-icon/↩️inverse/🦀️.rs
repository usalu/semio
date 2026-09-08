//! ↩️ `change-machine-icon` inverse — reconstructs the pre-change icon from BASE state; a machine
//! already absent from `base` has nothing to undo.

use crate::mutations::Process3dMutation;
use crate::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ChangeMachineIcon, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
    base.workshop.machines.iter().find(|machine| machine.id == payload.id).map(|machine| vec![Process3dMutation::ChangeMachineIcon(super::ChangeMachineIcon { id: payload.id.clone(), new_icon_id: machine.icon_id.clone() })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
