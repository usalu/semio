//! 🔺️ `change-anchor-h-ef-mm` sparse diff construction — writes only `En1992Diff.anchor_h_ef_mm` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_anchor_h_ef_mm::mutation::ChangeAnchorHEfMm;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeAnchorHEfMm, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_anchor_h_ef_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Anchor h ef mm must be a finite number.", Vec::<String>::new());
    }
    if base.anchor_h_ef_mm == payload.new_anchor_h_ef_mm {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Anchor h ef mm already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { anchor_h_ef_mm: Some(payload.new_anchor_h_ef_mm.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
