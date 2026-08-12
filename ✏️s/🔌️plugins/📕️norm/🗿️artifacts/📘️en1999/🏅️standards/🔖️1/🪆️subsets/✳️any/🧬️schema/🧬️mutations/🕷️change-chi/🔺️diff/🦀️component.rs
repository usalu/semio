//! 🔺️ `change-chi` sparse diff construction — writes only `En1999Diff.chi` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_chi::mutation::ChangeChi;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeChi, _base: &En1999Snapshot) -> En1999Diff {
    En1999Diff { chi: Some(payload.new_chi.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
