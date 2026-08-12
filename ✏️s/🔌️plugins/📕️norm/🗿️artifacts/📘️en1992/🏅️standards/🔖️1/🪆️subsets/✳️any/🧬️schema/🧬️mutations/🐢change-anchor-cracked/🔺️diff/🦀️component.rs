//! 🔺️ `change-anchor-cracked` sparse diff construction — writes only `En1992Diff.anchor_cracked` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_anchor_cracked::mutation::ChangeAnchorCracked;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAnchorCracked, _base: &En1992Snapshot) -> En1992Diff {
    En1992Diff { anchor_cracked: Some(payload.new_anchor_cracked.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
