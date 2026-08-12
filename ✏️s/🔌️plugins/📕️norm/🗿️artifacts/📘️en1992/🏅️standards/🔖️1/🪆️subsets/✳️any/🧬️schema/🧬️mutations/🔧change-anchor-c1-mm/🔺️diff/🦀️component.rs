//! 🔺️ `change-anchor-c1-mm` sparse diff construction — writes only `En1992Diff.anchor_c1_mm` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_anchor_c1_mm::mutation::ChangeAnchorC1Mm;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAnchorC1Mm, _base: &En1992Snapshot) -> En1992Diff {
    En1992Diff { anchor_c1_mm: Some(payload.new_anchor_c1_mm.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
