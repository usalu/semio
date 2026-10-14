//! ↩️ `delete-machine` inverse — reconstructs a `create-machine` from BASE state (original list
//! position + full payload); a machine already absent from `base` has nothing to undo.

use crate::mutations::create_machine::CreateMachine;
use crate::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::standards::v1::subsets::any::schema::mutations::create_machine::CreateMachine>>(payload: &super::DeleteMachine, base: &Process3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    base.workshop.machines.iter().position(|machine| machine.id == payload.id).map(|index| vec![Op::from(CreateMachine { index, machine: base.workshop.machines[index].clone() })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
