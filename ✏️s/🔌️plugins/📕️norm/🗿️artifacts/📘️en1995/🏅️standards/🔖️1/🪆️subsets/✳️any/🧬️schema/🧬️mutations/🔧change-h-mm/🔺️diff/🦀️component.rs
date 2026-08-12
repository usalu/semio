//! 🔺️ `change-h-mm` sparse diff construction — writes only `En1995Diff.h_mm` from the payload.

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::change_h_mm::mutation::ChangeHMm;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeHMm, _base: &En1995Snapshot) -> En1995Diff {
    En1995Diff { h_mm: Some(payload.new_h_mm.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
