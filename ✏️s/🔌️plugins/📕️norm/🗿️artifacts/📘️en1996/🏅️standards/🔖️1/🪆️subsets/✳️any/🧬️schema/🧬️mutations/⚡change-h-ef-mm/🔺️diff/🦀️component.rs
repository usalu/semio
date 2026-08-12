//! 🔺️ `change-h-ef-mm` sparse diff construction — writes only `En1996Diff.h_ef_mm` from the payload.

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::change_h_ef_mm::mutation::ChangeHEfMm;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeHEfMm, _base: &En1996Snapshot) -> En1996Diff {
    En1996Diff { h_ef_mm: Some(payload.new_h_ef_mm.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
