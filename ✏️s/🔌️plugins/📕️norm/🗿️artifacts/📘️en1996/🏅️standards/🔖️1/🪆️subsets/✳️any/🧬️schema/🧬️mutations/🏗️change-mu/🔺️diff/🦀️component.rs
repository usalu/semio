//! 🔺️ `change-mu` sparse diff construction — writes only `En1996Diff.mu` from the payload.

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::change_mu::mutation::ChangeMu;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeMu, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if !payload.new_mu.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Mu must be a finite number.", Vec::<String>::new());
    }
    if base.mu == payload.new_mu {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Mu already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { mu: Some(payload.new_mu.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
