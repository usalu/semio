//! 🔺️ `change-annex` sparse diff construction — writes only `En1999Diff.annex` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_annex::mutation::ChangeAnnex;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAnnex, _base: &En1999Snapshot) -> En1999Diff {
    En1999Diff { annex: Some(payload.new_annex.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
