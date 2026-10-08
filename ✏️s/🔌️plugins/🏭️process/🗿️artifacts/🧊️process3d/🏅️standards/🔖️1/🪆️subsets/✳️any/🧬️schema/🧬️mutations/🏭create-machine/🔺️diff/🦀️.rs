//! 🔺️ `create-machine` sparse diff construction — a whole-`Workshop` value diff (the artifact's
//! `Process3dDiff.workshop` field is a whole-value replace, matching every other machine
//! mutation's diff shape), built directly from `base` + payload, never a snapshot clone. Fatal
//! `duplicate-id` on an existing machine id.

use crate::diff::{Process3dDiff, Process3dMachinePatch, Process3dMachinesDelta};
use crate::Process3dSnapshot;

//#region 🔖️Diff
/// 🏗️ Builds the new workshop value with the machine appended.
pub fn diff(payload: &super::CreateMachine, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
    if base.workshop.machines.iter().any(|machine| machine.id == payload.machine.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A machine with id \"{}\" already exists.", payload.machine.id), [payload.machine.id.clone()]);
    }
    let at = payload.index.min(base.workshop.machines.len());
    protocol::MutationOutcome::new(Process3dDiff { workshop: Some(Process3dMachinesDelta::insertion(at, payload.machine.clone())), ..Default::default() })
}
//#endregion 🔖️Diff
