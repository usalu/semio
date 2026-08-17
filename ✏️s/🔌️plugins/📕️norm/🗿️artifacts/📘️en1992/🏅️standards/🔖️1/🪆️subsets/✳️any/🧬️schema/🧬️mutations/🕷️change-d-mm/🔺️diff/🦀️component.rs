//! 🔺️ `change-d-mm` sparse diff construction — writes only `En1992Diff.d_mm` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_d_mm::mutation::ChangeDMm;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeDMm, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_d_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "D mm must be a finite number.", Vec::<String>::new());
    }
    if base.d_mm == payload.new_d_mm {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "D mm already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { d_mm: Some(payload.new_d_mm.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
