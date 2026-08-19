//! 🔺️ `change-anchor-d-mm` sparse diff construction — writes only `En1992Diff.anchor_d_mm` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_anchor_d_mm::mutation::ChangeAnchorDMm;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeAnchorDMm, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_anchor_d_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Anchor d mm must be a finite number.", Vec::<String>::new());
    }
    if base.anchor_d_mm == payload.new_anchor_d_mm {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Anchor d mm already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { anchor_d_mm: Some(payload.new_anchor_d_mm.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
