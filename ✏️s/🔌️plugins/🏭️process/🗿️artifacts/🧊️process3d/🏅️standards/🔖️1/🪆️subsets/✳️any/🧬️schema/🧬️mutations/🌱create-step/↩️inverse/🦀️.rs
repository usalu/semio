//! ↩️ `create-step` inverse.
//!
//! 🌉️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 4: `diff()` is a documented no-op
//! (see the sibling `🔺️diff/🦀️component.rs`'s doc comment) — nothing changed, so per the
//! sanctioned `MutationKind::inverse` contract ("a mutation with nothing to undo returns
//! `Vec::new()`"), there is nothing to invert.

use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::CreateStep, _base: &Process3dSnapshot) -> Vec<Process3dMutation> {
    Vec::new()
}
//#endregion 🔖️Inverse
