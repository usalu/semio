//! ↩️ `rename-machine` inverse — reconstructs the pre-rename label from BASE state; a machine
//! already absent from `base` has nothing to undo.

use crate::artifacts::process3d::mutations::rename_machine::mutation::RenameMachine;
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::Process3dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &RenameMachine, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
    base.workshop.machines.iter().find(|machine| machine.id == payload.id).map(|machine| vec![Process3dMutation::RenameMachine(RenameMachine { id: payload.id.clone(), new_label: machine.label.clone() })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
