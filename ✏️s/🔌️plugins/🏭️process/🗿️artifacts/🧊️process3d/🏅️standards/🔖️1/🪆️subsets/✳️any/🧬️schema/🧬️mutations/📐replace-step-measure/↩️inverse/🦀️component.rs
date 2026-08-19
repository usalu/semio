//! ↩️ `replace-step-measure` inverse.
//!
//! 🌉️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 4: DOCUMENTED NO-OP — see
//! `🌱create-step/↩️inverse/🦀️component.rs`'s doc comment for the full rationale.

use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::mutations::replace_step_measure::mutation::ReplaceStepMeasure;
use crate::artifacts::process3d::Process3dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ReplaceStepMeasure, _base: &Process3dSnapshot) -> Vec<Process3dMutation> {
    Vec::new()
}
//#endregion 🔖️Inverse
