//! 🔺️ `change-sheet-w-el-mm3` sparse diff construction — writes only `En1999Diff.sheet_w_el_mm3` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_sheet_w_el_mm3::mutation::ChangeSheetWElMm3;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeSheetWElMm3, _base: &En1999Snapshot) -> En1999Diff {
    En1999Diff { sheet_w_el_mm3: Some(payload.new_sheet_w_el_mm3.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
