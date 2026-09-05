//! 🔺️ `change-anchor-c1-mm` sparse diff construction — writes only `En1992Diff.anchor_c1_mm` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_anchor_c1_mm::ChangeAnchorC1Mm;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAnchorC1Mm, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_anchor_c1_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Anchor c1 mm must be a finite number.", Vec::<String>::new());
    }
    if base.anchor_c1_mm == payload.new_anchor_c1_mm {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Anchor c1 mm already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { anchor_c1_mm: Some(payload.new_anchor_c1_mm.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
