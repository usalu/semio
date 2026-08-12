//! ↩️ `create-machine` inverse — undo of a create is always a `delete-machine` by the created id.

use crate::artifacts::process3d::mutations::delete_machine::mutation::DeleteMachine;
use crate::artifacts::process3d::mutations::machines::mutation::CreateMachine;
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::Process3dSnapshot;

//#region 🔖️Inverse
/// ↩️ Undoing a create is deleting the same machine back out, by its own id.
pub fn inverse(payload: &CreateMachine, _base: &Process3dSnapshot) -> Vec<Process3dMutation> {
    vec![Process3dMutation::DeleteMachine(DeleteMachine { id: payload.machine.id.clone() })]
}
//#endregion 🔖️Inverse
