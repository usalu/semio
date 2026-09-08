//! ↩️ `delete-machine` inverse — reconstructs a `create-machine` from BASE state (original list
//! position + full payload); a machine already absent from `base` has nothing to undo.

use crate::mutations::create_machine::CreateMachine;
use crate::mutations::Process3dMutation;
use crate::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DeleteMachine, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
    base.workshop.machines.iter().position(|machine| machine.id == payload.id).map(|index| vec![Process3dMutation::CreateMachine(CreateMachine { index, machine: base.workshop.machines[index].clone() })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
