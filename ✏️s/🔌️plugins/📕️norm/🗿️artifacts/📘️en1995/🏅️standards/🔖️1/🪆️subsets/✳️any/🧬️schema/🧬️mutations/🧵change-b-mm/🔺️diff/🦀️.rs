//! 🔺️ `change-b-mm` sparse diff construction — writes only `En1995Diff.b_mm` from the payload.

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::change_b_mm::ChangeBMm;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeBMm, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if !payload.new_b_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "B mm must be a finite number.", Vec::<String>::new());
    }
    if base.b_mm == payload.new_b_mm {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "B mm already has this value.");
    }
    protocol::MutationOutcome::new(En1995Diff { b_mm: Some(payload.new_b_mm.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
