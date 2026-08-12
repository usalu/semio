//! 🔺️ `change-machine-icon` sparse diff construction — a whole-`Workshop` value diff, built
//! directly from `base` + payload, never a snapshot clone.

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::change_machine_icon::mutation::ChangeMachineIcon;
use crate::artifacts::process3d::{Process3dSnapshot, Workshop};

//#region 🔖️Diff
pub fn diff(payload: &ChangeMachineIcon, base: &Process3dSnapshot) -> Process3dDiff {
    let mut machines = base.workshop.machines.clone();
    if let Some(machine) = machines.iter_mut().find(|machine| machine.id == payload.id) {
        machine.icon_id = payload.new_icon_id.clone();
    }
    Process3dDiff { workshop: Some(Workshop { machines }), ..Default::default() }
}
//#endregion 🔖️Diff
