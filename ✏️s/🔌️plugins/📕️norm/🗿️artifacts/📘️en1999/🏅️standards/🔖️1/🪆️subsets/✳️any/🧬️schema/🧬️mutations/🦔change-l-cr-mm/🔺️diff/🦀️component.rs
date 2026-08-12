//! 🔺️ `change-l-cr-mm` sparse diff construction — writes only `En1999Diff.l_cr_mm` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_l_cr_mm::mutation::ChangeLCrMm;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeLCrMm, _base: &En1999Snapshot) -> En1999Diff {
    En1999Diff { l_cr_mm: Some(payload.new_l_cr_mm.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
