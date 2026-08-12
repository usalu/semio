//! 🔺️ `change-annex` sparse diff construction — writes only `En1997Diff.annex` from the payload.

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::change_annex::mutation::ChangeAnnex;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAnnex, _base: &En1997Snapshot) -> En1997Diff {
    En1997Diff { annex: Some(payload.new_annex.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
