//! 🔺️ `change-d-mm` — sparse diff construction.

use super::mutation::ChangeDMm;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeDMm, _base: &En1994Snapshot) -> En1994Diff {
    En1994Diff { d_mm: Some(payload.new_d_mm), ..Default::default() }
}
//#endregion 🔖️Diff
