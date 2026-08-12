//! 🔺️ `change-d-mm` sparse diff construction — writes only `En1992Diff.d_mm` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_d_mm::mutation::ChangeDMm;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeDMm, _base: &En1992Snapshot) -> En1992Diff {
    En1992Diff { d_mm: Some(payload.new_d_mm.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
