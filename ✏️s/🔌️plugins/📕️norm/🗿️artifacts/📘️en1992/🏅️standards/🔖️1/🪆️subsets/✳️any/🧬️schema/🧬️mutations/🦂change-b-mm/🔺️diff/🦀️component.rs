//! 🔺️ `change-b-mm` sparse diff construction — writes only `En1992Diff.b_mm` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_b_mm::mutation::ChangeBMm;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeBMm, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_b_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "B mm must be a finite number.", Vec::<String>::new());
    }
    if base.b_mm == payload.new_b_mm {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "B mm already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { b_mm: Some(payload.new_b_mm.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
