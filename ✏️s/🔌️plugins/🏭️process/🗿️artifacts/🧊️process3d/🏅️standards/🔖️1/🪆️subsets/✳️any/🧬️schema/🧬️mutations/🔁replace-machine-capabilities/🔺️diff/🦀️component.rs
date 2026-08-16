//! 🔺️ `replace-machine-capabilities` sparse diff construction — a whole-`Workshop` value diff,
//! built directly from `base` + payload, never a snapshot clone. Error `target-missing` when the
//! machine is absent, Warning `no-op` when the capability list is unchanged.

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::replace_machine_capabilities::mutation::ReplaceMachineCapabilities;
use crate::artifacts::process3d::{Process3dSnapshot, Workshop};

//#region 🔖️Diff
pub fn diff(payload: &ReplaceMachineCapabilities, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
    let Some(existing) = base.workshop.machines.iter().find(|machine| machine.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Machine \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.capabilities == payload.new_capabilities {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Machine \"{}\" capabilities are unchanged.", payload.id));
    }
    let mut machines = base.workshop.machines.clone();
    if let Some(machine) = machines.iter_mut().find(|machine| machine.id == payload.id) {
        machine.capabilities = payload.new_capabilities.clone();
    }
    protocol::MutationOutcome::new(Process3dDiff { workshop: Some(Workshop { machines }), ..Default::default() })
}
//#endregion 🔖️Diff
