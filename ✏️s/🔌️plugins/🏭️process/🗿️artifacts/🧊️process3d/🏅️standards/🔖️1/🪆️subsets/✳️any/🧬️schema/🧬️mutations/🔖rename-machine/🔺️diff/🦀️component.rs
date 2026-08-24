//! 🔺️ `rename-machine` sparse diff construction — a whole-`Workshop` value diff, built directly
//! from `base` + payload, never a snapshot clone. Error `target-missing` when the machine is
//! absent, Warning `no-op` when the new label equals the old (machine `label` is a non-unique
//! display string, not a key, so no `duplicate-id` case applies here).

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::rename_machine::mutation::RenameMachine;
use crate::artifacts::process3d::{Process3dSnapshot, Workshop};

//#region 🔖️Diff
pub fn diff(payload: &RenameMachine, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
    let Some(existing) = base.workshop.machines.iter().find(|machine| machine.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Machine \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.label == payload.new_label {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Machine \"{}\" is already named \"{}\".", payload.id, payload.new_label));
    }
    let mut machines = base.workshop.machines.clone();
    if let Some(machine) = machines.iter_mut().find(|machine| machine.id == payload.id) {
        machine.label = payload.new_label.clone();
    }
    protocol::MutationOutcome::new(Process3dDiff { workshop: Some(Workshop { machines }), ..Default::default() })
}
//#endregion 🔖️Diff
