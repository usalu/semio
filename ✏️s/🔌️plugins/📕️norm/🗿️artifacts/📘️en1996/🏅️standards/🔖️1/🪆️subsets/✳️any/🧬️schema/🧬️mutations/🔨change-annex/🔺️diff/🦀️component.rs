//! 🔺️ `change-annex` sparse diff construction — writes only `En1996Diff.annex` from the payload.

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::change_annex::mutation::ChangeAnnex;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAnnex, _base: &En1996Snapshot) -> En1996Diff {
    En1996Diff { annex: Some(payload.new_annex.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
