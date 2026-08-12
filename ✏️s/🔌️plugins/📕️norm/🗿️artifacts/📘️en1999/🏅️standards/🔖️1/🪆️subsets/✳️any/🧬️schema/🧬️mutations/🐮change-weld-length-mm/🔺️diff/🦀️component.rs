//! 🔺️ `change-weld-length-mm` sparse diff construction — writes only `En1999Diff.weld_length_mm` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_weld_length_mm::mutation::ChangeWeldLengthMm;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeWeldLengthMm, _base: &En1999Snapshot) -> En1999Diff {
    En1999Diff { weld_length_mm: Some(payload.new_weld_length_mm.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
