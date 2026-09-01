//! 🔺️ `change-permanent-action` — sparse diff construction; writes only `En1990Diff.g_k`.

use super::ChangePermanentAction;
use crate::artifacts::en1990::{En1990Diff, En1990Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangePermanentAction, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    if !payload.new_g_k.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Permanent action G_k must be a finite number.", Vec::<String>::new());
    }
    if base.g_k == payload.new_g_k {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Permanent action already has this value.");
    }
    protocol::MutationOutcome::new(En1990Diff { g_k: Some(payload.new_g_k), ..Default::default() })
}
//#endregion 🔖️Diff
