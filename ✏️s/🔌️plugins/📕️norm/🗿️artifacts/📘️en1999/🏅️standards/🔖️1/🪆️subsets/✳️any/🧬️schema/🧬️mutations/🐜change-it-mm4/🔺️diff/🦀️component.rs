//! 🔺️ `change-it-mm4` sparse diff construction — writes only `En1999Diff.i_t_mm4` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_i_t_mm4::mutation::ChangeITMm4;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeITMm4, _base: &En1999Snapshot) -> En1999Diff {
    En1999Diff { i_t_mm4: Some(payload.new_i_t_mm4.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
