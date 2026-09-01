//! 🔺️ `change-strict-mode` — sparse diff construction.

use super::ChangeStrictMode;
use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeStrictMode, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    if base.strict_mode == payload.new_strict_mode {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Strict mode already has this value.");
    }
    protocol::MutationOutcome::new(Vdi3805Diff { strict_mode: Some(payload.new_strict_mode.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
