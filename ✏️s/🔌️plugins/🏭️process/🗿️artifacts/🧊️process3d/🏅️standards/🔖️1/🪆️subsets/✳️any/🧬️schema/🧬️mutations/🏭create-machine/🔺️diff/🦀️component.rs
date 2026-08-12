//! 🔺️ `create-machine` sparse diff construction — a whole-`Workshop` value diff (the artifact's
//! `Process3dDiff.workshop` field is a whole-value replace, matching every other machine
//! mutation's diff shape), built directly from `base` + payload, never a snapshot clone.

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::create_machine::mutation::CreateMachine;
use crate::artifacts::process3d::{Process3dSnapshot, Workshop};

//#region 🔖️Diff
/// 🏗️ Builds the new workshop value with the machine appended.
pub fn diff(payload: &CreateMachine, base: &Process3dSnapshot) -> Process3dDiff {
    let mut machines = base.workshop.machines.clone();
    machines.push(payload.machine.clone());
    Process3dDiff { workshop: Some(Workshop { machines }), ..Default::default() }
}
//#endregion 🔖️Diff
