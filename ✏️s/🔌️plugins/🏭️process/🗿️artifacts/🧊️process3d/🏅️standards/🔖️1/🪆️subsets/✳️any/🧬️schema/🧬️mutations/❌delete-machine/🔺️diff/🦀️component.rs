//! 🔺️ `delete-machine` sparse diff construction — a whole-`Workshop` value diff, built directly
//! from `base` + payload, never a snapshot clone. Error `target-missing` when the machine is absent.

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::delete_machine::mutation::DeleteMachine;
use crate::artifacts::process3d::{Process3dSnapshot, Workshop};

//#region 🔖️Diff
pub fn diff(payload: &DeleteMachine, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
    if !base.workshop.machines.iter().any(|machine| machine.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Machine \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let mut machines = base.workshop.machines.clone();
    machines.retain(|machine| machine.id != payload.id);
    protocol::MutationOutcome::new(Process3dDiff { workshop: Some(Workshop { machines }), ..Default::default() })
}
//#endregion 🔖️Diff
