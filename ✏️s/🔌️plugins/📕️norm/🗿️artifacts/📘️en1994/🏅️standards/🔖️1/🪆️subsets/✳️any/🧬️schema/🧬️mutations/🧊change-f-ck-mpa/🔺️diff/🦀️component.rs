//! 🔺️ `change-f-ck-mpa` — sparse diff construction.

use super::mutation::ChangeFCkMpa;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeFCkMpa, _base: &En1994Snapshot) -> En1994Diff {
    En1994Diff { f_ck_mpa: Some(payload.new_f_ck_mpa), ..Default::default() }
}
//#endregion 🔖️Diff
