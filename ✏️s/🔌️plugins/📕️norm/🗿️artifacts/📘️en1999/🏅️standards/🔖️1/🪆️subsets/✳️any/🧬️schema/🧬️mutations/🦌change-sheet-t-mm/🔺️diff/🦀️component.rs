//! 🔺️ `change-sheet-t-mm` sparse diff construction — writes only `En1999Diff.sheet_t_mm` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_sheet_t_mm::mutation::ChangeSheetTMm;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeSheetTMm, _base: &En1999Snapshot) -> En1999Diff {
    En1999Diff { sheet_t_mm: Some(payload.new_sheet_t_mm.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
