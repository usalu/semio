//! ↩️ `create-machine` inverse — undo of a create is always a `delete-machine` by the created id.

use crate::mutations::delete_machine::DeleteMachine;
use crate::mutations::Process3dMutation;
use crate::Process3dSnapshot;

//#region 🔖️Inverse
/// ↩️ Undoing a create is deleting the same machine back out, by its own id.
pub fn inverse(payload: &super::CreateMachine, _base: &Process3dSnapshot) -> Result<Vec<Process3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Process3dMutation::DeleteMachine(DeleteMachine { id: payload.machine.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
