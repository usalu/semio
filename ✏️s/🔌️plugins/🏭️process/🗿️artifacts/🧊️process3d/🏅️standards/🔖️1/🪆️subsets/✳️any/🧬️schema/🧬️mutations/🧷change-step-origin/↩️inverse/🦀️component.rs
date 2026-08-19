//! ↩️ `change-step-origin` inverse.
//!
//! 🌉️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 4: DOCUMENTED NO-OP — see
//! `🌱create-step/↩️inverse/🦀️component.rs`'s doc comment for the full rationale.

use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::mutations::change_step_origin::mutation::ChangeStepOrigin;
use crate::artifacts::process3d::Process3dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeStepOrigin, _base: &Process3dSnapshot) -> Vec<Process3dMutation> {
    Vec::new()
}
//#endregion 🔖️Inverse
