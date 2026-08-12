//! 🔺️ `change-annex` sparse diff construction — writes only `En1990Diff.annex` from the payload.

use crate::artifacts::en1990::mutations::set_snapshot::mutation::ChangeAnnex;
use crate::artifacts::en1990::{En1990Diff, En1990Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeAnnex, _base: &En1990Snapshot) -> En1990Diff {
    En1990Diff { annex: Some(payload.new_annex), ..Default::default() }
}
//#endregion 🔖️Diff
