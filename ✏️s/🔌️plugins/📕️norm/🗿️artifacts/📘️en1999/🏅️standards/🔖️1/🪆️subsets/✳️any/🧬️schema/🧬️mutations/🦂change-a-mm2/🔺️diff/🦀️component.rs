//! 🔺️ `change-a-mm2` sparse diff construction — writes only `En1999Diff.a_mm2` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_a_mm2::mutation::ChangeAMm2;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAMm2, _base: &En1999Snapshot) -> En1999Diff {
    En1999Diff { a_mm2: Some(payload.new_a_mm2.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
