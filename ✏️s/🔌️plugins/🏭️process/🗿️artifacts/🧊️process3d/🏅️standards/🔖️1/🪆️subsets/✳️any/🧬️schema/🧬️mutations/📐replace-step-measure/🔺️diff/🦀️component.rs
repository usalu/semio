//! 🔺️ `replace-step-measure` sparse diff construction.
//!
//! 🌉️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 4: DOCUMENTED NO-OP — see
//! `🌱create-step/🔺️diff/🦀️component.rs`'s doc comment for the full rationale.

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::replace_step_measure::mutation::ReplaceStepMeasure;
use crate::artifacts::process3d::Process3dSnapshot;

//#region 🔖️Diff
/// 🚧️ Documented no-op — see file doc comment. Surfaced as Warning `mutation.no-op`.
pub fn diff(_payload: &ReplaceStepMeasure, _base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
    protocol::MutationOutcome::empty().warn("mutation.no-op", "Replacing step measure is a documented no-op pending a link resolver for the composed steps child.".to_string())
}
//#endregion 🔖️Diff
