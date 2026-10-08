//! 🔺️ `delete-machine` sparse diff construction — a whole-`Workshop` value diff, built directly
//! from `base` + payload, never a snapshot clone. Error `target-missing` when the machine is absent.

use crate::diff::{Process3dDiff, Process3dMachinePatch, Process3dMachinesDelta};
use crate::Process3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteMachine, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
    let Some(index) = base.workshop.machines.iter().position(|machine| machine.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Machine \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(Process3dDiff { workshop: Some(Process3dMachinesDelta::removal(&base.workshop.machines, index)), ..Default::default() })
}
//#endregion 🔖️Diff
