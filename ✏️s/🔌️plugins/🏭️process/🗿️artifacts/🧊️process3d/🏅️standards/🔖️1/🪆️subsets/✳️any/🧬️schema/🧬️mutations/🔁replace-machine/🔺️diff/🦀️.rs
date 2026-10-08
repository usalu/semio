//! 🔺️ `replace-machine-capabilities` sparse diff construction — a whole-`Workshop` value diff,
//! built directly from `base` + payload, never a snapshot clone. Error `target-missing` when the
//! machine is absent, Warning `no-op` when the capability list is unchanged.

use crate::diff::{Process3dDiff, Process3dMachinePatch, Process3dMachinesDelta};
use crate::Process3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ReplaceMachineCapabilities, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
    let Some(existing) = base.workshop.machines.iter().find(|machine| machine.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Machine \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.capabilities == payload.new_capabilities {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Machine \"{}\" capabilities are unchanged.", payload.id));
    }
    protocol::MutationOutcome::new(Process3dDiff { workshop: Some(Process3dMachinesDelta { patched: vec![Process3dMachinePatch { id: payload.id.clone(), capabilities: Some(payload.new_capabilities.clone()), ..Default::default() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
