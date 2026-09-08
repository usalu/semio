//! 🔺️ `change-anchor-cracked` sparse diff construction — writes only `En1992Diff.anchor_cracked` from the payload.

use crate::diff::En1992Diff;
use crate::mutations::change_anchor_cracked::ChangeAnchorCracked;
use crate::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAnchorCracked, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if base.anchor_cracked == payload.new_anchor_cracked {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Anchor cracked already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { anchor_cracked: Some(payload.new_anchor_cracked), ..Default::default() })
}
//#endregion 🔖️Diff
