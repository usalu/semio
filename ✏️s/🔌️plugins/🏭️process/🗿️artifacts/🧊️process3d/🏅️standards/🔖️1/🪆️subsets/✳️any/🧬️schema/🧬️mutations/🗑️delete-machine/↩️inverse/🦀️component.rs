//! ↩️ `delete-machine` inverse — reconstructs a `create-machine` from BASE state (original list
//! position + full payload); a machine already absent from `base` has nothing to undo.

use crate::artifacts::process3d::mutations::delete_machine::mutation::DeleteMachine;
use crate::artifacts::process3d::mutations::machines::mutation::CreateMachine;
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteMachine, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
    base.workshop
        .machines
        .iter()
        .position(|machine| machine.id == payload.id)
        .map(|index| vec![Process3dMutation::CreateMachine(CreateMachine { index, machine: base.workshop.machines[index].clone() })])
        .unwrap_or_default()
}
//#endregion 🔖️Inverse
