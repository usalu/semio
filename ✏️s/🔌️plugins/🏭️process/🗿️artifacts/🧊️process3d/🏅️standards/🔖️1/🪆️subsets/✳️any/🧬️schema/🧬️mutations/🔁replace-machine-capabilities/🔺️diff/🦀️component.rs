//! 🔺️ `replace-machine-capabilities` sparse diff construction — a whole-`Workshop` value diff,
//! built directly from `base` + payload, never a snapshot clone.

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::replace_machine_capabilities::mutation::ReplaceMachineCapabilities;
use crate::artifacts::process3d::{Process3dSnapshot, Workshop};

//#region 🔖️Diff
pub fn diff(payload: &ReplaceMachineCapabilities, base: &Process3dSnapshot) -> Process3dDiff {
    let mut machines = base.workshop.machines.clone();
    if let Some(machine) = machines.iter_mut().find(|machine| machine.id == payload.id) {
        machine.capabilities = payload.new_capabilities.clone();
    }
    Process3dDiff { workshop: Some(Workshop { machines }), ..Default::default() }
}
//#endregion 🔖️Diff
