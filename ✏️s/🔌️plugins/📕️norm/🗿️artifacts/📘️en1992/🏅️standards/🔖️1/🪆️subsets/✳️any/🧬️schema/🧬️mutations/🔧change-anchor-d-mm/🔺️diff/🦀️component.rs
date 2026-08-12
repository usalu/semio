//! 🔺️ `change-anchor-d-mm` sparse diff construction — writes only `En1992Diff.anchor_d_mm` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_anchor_d_mm::mutation::ChangeAnchorDMm;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAnchorDMm, _base: &En1992Snapshot) -> En1992Diff {
    En1992Diff { anchor_d_mm: Some(payload.new_anchor_d_mm.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
