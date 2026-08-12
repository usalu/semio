//! 🔺️ `change-b-mm` sparse diff construction — writes only `En1992Diff.b_mm` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_b_mm::mutation::ChangeBMm;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeBMm, _base: &En1992Snapshot) -> En1992Diff {
    En1992Diff { b_mm: Some(payload.new_b_mm.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
