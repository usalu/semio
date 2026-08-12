//! 🔺️ `change-annex` sparse diff construction — writes only `En1992Diff.annex` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::set_snapshot::mutation::ChangeAnnex;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAnnex, _base: &En1992Snapshot) -> En1992Diff {
    En1992Diff { annex: Some(payload.new_annex.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
