//! ↩️ `create-machine` inverse — undo of a create is always a `delete-machine` by the created id.

use crate::mutations::delete_machine::DeleteMachine;
use crate::Process3dSnapshot;

//#region 🔖️Inverse
/// ↩️ Undoing a create is deleting the same machine back out, by its own id.
pub fn inverse<Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_machine::DeleteMachine>>(payload: &super::CreateMachine, _base: &Process3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Op::from(DeleteMachine { id: payload.machine.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
