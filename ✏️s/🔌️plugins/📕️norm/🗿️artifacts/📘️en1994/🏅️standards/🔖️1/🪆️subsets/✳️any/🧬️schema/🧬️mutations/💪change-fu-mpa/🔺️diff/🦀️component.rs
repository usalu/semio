//! 🔺️ `change-fu-mpa` — sparse diff construction.

use super::mutation::ChangeFUMpa;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeFUMpa, _base: &En1994Snapshot) -> En1994Diff {
    En1994Diff { f_u_mpa: Some(payload.new_f_u_mpa), ..Default::default() }
}
//#endregion 🔖️Diff
