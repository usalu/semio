//! 🔺️ `change-fy-mpa` — sparse diff construction.

use super::mutation::ChangeFYMpa;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeFYMpa, _base: &En1994Snapshot) -> En1994Diff {
    En1994Diff { f_y_mpa: Some(payload.new_f_y_mpa), ..Default::default() }
}
//#endregion 🔖️Diff
