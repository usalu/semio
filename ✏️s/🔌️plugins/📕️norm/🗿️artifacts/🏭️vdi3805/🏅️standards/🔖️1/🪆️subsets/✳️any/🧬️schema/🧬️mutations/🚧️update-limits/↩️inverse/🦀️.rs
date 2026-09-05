//! ↩️ `update-limits` — undo restores BASE's whole limits facet.

use super::UpdateLimits;
use crate::artifacts::vdi3805::{Vdi3805Mutation, Vdi3805Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &UpdateLimits, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
    vec![Vdi3805Mutation::UpdateLimits(UpdateLimits { new_limits: base.limits })]
}
//#endregion 🔖️Inverse
