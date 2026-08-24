//! ↩️ `rename-step` inverse.
//!
//! 🌉️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 4: DOCUMENTED NO-OP — see
//! `🌱create-step/↩️inverse/🦀️component.rs`'s doc comment for the full rationale.

use crate::artifacts::process3d::mutations::rename_step::mutation::RenameStep;
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &RenameStep, _base: &Process3dSnapshot) -> Vec<Process3dMutation> {
    Vec::new()
}
//#endregion 🔖️Inverse
