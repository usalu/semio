//! 🔺️ `delete-machine` sparse diff construction — a whole-`Workshop` value diff, built directly
//! from `base` + payload, never a snapshot clone.

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::delete_machine::mutation::DeleteMachine;
use crate::artifacts::process3d::{Process3dSnapshot, Workshop};

//#region 🔖️Diff
pub fn diff(payload: &DeleteMachine, base: &Process3dSnapshot) -> Process3dDiff {
    let mut machines = base.workshop.machines.clone();
    machines.retain(|machine| machine.id != payload.id);
    Process3dDiff { workshop: Some(Workshop { machines }), ..Default::default() }
}
//#endregion 🔖️Diff
