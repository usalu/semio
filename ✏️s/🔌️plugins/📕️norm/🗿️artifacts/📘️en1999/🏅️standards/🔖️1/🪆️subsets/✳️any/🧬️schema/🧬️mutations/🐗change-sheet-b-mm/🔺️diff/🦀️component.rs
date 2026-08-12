//! 🔺️ `change-sheet-b-mm` sparse diff construction — writes only `En1999Diff.sheet_b_mm` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_sheet_b_mm::mutation::ChangeSheetBMm;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeSheetBMm, _base: &En1999Snapshot) -> En1999Diff {
    En1999Diff { sheet_b_mm: Some(payload.new_sheet_b_mm.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
