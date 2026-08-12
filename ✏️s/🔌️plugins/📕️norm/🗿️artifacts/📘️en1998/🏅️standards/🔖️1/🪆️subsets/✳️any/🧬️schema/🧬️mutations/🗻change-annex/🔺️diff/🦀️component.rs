//! 🔺️ `change-annex` sparse diff construction — writes only `En1998Diff.annex` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_annex::mutation::ChangeAnnex;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAnnex, _base: &En1998Snapshot) -> En1998Diff {
    En1998Diff { annex: Some(payload.new_annex.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
