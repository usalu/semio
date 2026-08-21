//! ↩️ `change-machine-icon` inverse — reconstructs the pre-change icon from BASE state; a machine
//! already absent from `base` has nothing to undo.

use crate::artifacts::process3d::mutations::change_machine_icon::mutation::ChangeMachineIcon;
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::Process3dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &ChangeMachineIcon, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
    base.workshop.machines.iter().find(|machine| machine.id == payload.id).map(|machine| vec![Process3dMutation::ChangeMachineIcon(ChangeMachineIcon { id: payload.id.clone(), new_icon_id: machine.icon_id.clone() })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
