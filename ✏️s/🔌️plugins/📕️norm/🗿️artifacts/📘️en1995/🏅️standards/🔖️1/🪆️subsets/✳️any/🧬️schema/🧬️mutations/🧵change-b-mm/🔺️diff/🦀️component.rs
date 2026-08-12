//! 🔺️ `change-b-mm` sparse diff construction — writes only `En1995Diff.b_mm` from the payload.

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::change_b_mm::mutation::ChangeBMm;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeBMm, _base: &En1995Snapshot) -> En1995Diff {
    En1995Diff { b_mm: Some(payload.new_b_mm.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
