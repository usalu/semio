//! 🔺️ `change-machine-icon` sparse diff construction — a whole-`Workshop` value diff, built
//! directly from `base` + payload, never a snapshot clone. Error `target-missing` when the machine
//! is absent, Warning `no-op` when the icon is unchanged.

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::{Process3dSnapshot, Workshop};

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeMachineIcon, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
    let Some(existing) = base.workshop.machines.iter().find(|machine| machine.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Machine \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.icon_id == payload.new_icon_id {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Machine \"{}\" icon is already \"{}\".", payload.id, payload.new_icon_id));
    }
    let mut machines = base.workshop.machines.clone();
    if let Some(machine) = machines.iter_mut().find(|machine| machine.id == payload.id) {
        machine.icon_id = payload.new_icon_id.clone();
    }
    protocol::MutationOutcome::new(Process3dDiff { workshop: Some(Workshop { machines }), ..Default::default() })
}
//#endregion 🔖️Diff
