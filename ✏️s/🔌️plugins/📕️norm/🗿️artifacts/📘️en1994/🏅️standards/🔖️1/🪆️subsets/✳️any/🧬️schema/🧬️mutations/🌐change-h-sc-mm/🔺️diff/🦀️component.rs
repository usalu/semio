//! 🔺️ `change-h-sc-mm` — sparse diff construction.

use super::mutation::ChangeHScMm;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeHScMm, _base: &En1994Snapshot) -> En1994Diff {
    En1994Diff { h_sc_mm: Some(payload.new_h_sc_mm), ..Default::default() }
}
//#endregion 🔖️Diff
