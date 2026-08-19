//! 🔺️ `change-anchor-as-mm2` sparse diff construction — writes only `En1992Diff.anchor_a_s_mm2` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_anchor_a_s_mm2::mutation::ChangeAnchorASMm2;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeAnchorASMm2, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_anchor_a_s_mm2.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Anchor as mm2 must be a finite number.", Vec::<String>::new());
    }
    if base.anchor_a_s_mm2 == payload.new_anchor_a_s_mm2 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Anchor as mm2 already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { anchor_a_s_mm2: Some(payload.new_anchor_a_s_mm2.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
