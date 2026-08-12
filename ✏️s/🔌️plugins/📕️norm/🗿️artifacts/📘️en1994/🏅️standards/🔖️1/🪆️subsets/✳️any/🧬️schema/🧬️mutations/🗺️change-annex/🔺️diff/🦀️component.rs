//! 🔺️ `change-annex` — sparse diff construction.

use super::mutation::ChangeAnnex;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeAnnex, _base: &En1994Snapshot) -> En1994Diff {
    En1994Diff { annex: Some(payload.new_annex), ..Default::default() }
}
//#endregion 🔖️Diff
