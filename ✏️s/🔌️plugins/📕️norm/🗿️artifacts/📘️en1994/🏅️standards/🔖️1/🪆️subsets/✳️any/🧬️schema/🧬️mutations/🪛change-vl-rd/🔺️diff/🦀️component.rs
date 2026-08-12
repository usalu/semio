//! 🔺️ `change-vl-rd` — sparse diff construction.

use super::mutation::ChangeVLRd;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeVLRd, _base: &En1994Snapshot) -> En1994Diff {
    En1994Diff { v_l_rd: Some(payload.new_v_l_rd), ..Default::default() }
}
//#endregion 🔖️Diff
