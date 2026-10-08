//! 🔺️ `rename-machine` sparse diff construction — a whole-`Workshop` value diff, built directly
//! from `base` + payload, never a snapshot clone. Error `target-missing` when the machine is
//! absent, Warning `no-op` when the new label equals the old (machine `label` is a non-unique
//! display string, not a key, so no `duplicate-id` case applies here).

use crate::diff::{Process3dDiff, Process3dMachinePatch, Process3dMachinesDelta};
use crate::Process3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::RenameMachine, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
    let Some(existing) = base.workshop.machines.iter().find(|machine| machine.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Machine \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.label == payload.new_label {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Machine \"{}\" is already named \"{}\".", payload.id, payload.new_label));
    }
    protocol::MutationOutcome::new(Process3dDiff { workshop: Some(Process3dMachinesDelta::modification(payload.id.clone(), Process3dMachinePatch { label: Some(payload.new_label.clone()), ..Default::default() })), ..Default::default() })
}
//#endregion 🔖️Diff
