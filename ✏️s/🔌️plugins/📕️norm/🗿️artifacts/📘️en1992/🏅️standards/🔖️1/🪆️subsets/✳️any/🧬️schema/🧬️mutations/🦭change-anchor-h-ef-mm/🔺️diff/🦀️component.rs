//! 🔺️ `change-anchor-h-ef-mm` sparse diff construction — writes only `En1992Diff.anchor_h_ef_mm` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_anchor_h_ef_mm::mutation::ChangeAnchorHEfMm;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAnchorHEfMm, _base: &En1992Snapshot) -> En1992Diff {
    En1992Diff { anchor_h_ef_mm: Some(payload.new_anchor_h_ef_mm.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
