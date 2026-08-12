//! 🔺️ `change-m-pl-rd` — sparse diff construction.

use super::mutation::ChangeMPlRd;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeMPlRd, _base: &En1994Snapshot) -> En1994Diff {
    En1994Diff { m_pl_rd: Some(payload.new_m_pl_rd), ..Default::default() }
}
//#endregion 🔖️Diff
