//! 🔺️ `change-annex` — sparse diff construction.

use super::mutation::ChangeAnnex;
use crate::artifacts::en1993::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeAnnex, _base: &En1993Snapshot) -> En1993Diff {
    En1993Diff { annex: Some(payload.new_annex), ..Default::default() }
}
//#endregion 🔖️Diff
