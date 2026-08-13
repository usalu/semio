//! 🔺️ `delete-step` sparse diff construction.
//!
//! 🌉️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 4: DOCUMENTED NO-OP — see
//! `🌱create-step/🔺️diff/🦀️component.rs`'s doc comment for the full rationale (composed `steps`
//! child, no resolver).

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::delete_step::mutation::DeleteStep;
use crate::artifacts::process3d::Process3dSnapshot;

//#region 🔖️Diff
pub fn diff(_payload: &DeleteStep, _base: &Process3dSnapshot) -> Process3dDiff {
    Process3dDiff::default()
}
//#endregion 🔖️Diff
