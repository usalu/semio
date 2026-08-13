//! 🔺️ `reorder-steps` sparse diff construction.
//!
//! 🌉️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 4: DOCUMENTED NO-OP — see
//! `🌱create-step/🔺️diff/🦀️component.rs`'s doc comment for the full rationale.

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::reorder_steps::mutation::ReorderSteps;
use crate::artifacts::process3d::Process3dSnapshot;

//#region 🔖️Diff
pub fn diff(_payload: &ReorderSteps, _base: &Process3dSnapshot) -> Process3dDiff {
    Process3dDiff::default()
}
//#endregion 🔖️Diff
