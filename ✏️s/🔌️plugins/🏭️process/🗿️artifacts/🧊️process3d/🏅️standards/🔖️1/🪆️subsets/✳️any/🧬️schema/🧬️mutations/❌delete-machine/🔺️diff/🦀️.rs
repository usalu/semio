//! 🔺️ `delete-machine` sparse diff construction — a whole-`Workshop` value diff, built directly
//! from `base` + payload, never a snapshot clone. Error `target-missing` when the machine is absent.

use crate::diff::{Process3dDiff, Process3dMachinePatch, Process3dMachinesDelta};
use crate::Process3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteMachine, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
    if !base.workshop.machines.iter().any(|machine| machine.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Machine \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Process3dDiff { workshop: Some(Process3dMachinesDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
