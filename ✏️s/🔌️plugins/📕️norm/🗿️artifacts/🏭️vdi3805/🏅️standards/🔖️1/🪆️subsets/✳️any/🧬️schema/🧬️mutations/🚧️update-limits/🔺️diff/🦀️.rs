//! 🔺️ `update-limits` — sparse diff construction.

use super::UpdateLimits;
use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateLimits, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    if base.limits == payload.new_limits {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Limits already has this value.");
    }
    protocol::MutationOutcome::new(Vdi3805Diff { limits: Some(payload.new_limits.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
