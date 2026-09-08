//! 🔺️ `change-mu` sparse diff construction — writes only `En1996Diff.mu` from the payload.

use crate::diff::En1996Diff;
use crate::mutations::change_mu::ChangeMu;
use crate::En1996Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeMu, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if !payload.new_mu.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Mu must be a finite number.", Vec::<String>::new());
    }
    if base.mu == payload.new_mu {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Mu already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { mu: Some(payload.new_mu), ..Default::default() })
}
//#endregion 🔖️Diff
