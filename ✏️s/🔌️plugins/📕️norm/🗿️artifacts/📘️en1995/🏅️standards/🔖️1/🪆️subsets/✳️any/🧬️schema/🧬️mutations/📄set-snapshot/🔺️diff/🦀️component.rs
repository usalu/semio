//! 🔺️ `change-annex` sparse diff construction — writes only `En1995Diff.annex` from the payload.

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::set_snapshot::mutation::ChangeAnnex;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAnnex, _base: &En1995Snapshot) -> En1995Diff {
    En1995Diff { annex: Some(payload.new_annex.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
