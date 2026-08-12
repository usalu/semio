//! 🔺️ `change-w-el-mm3` sparse diff construction — writes only `En1999Diff.w_el_mm3` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_w_el_mm3::mutation::ChangeWElMm3;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeWElMm3, _base: &En1999Snapshot) -> En1999Diff {
    En1999Diff { w_el_mm3: Some(payload.new_w_el_mm3.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
