//! 🔺️ `change-m-pla` — sparse diff construction.

use super::mutation::ChangeMPla;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeMPla, _base: &En1994Snapshot) -> En1994Diff {
    En1994Diff { m_pla: Some(payload.new_m_pla), ..Default::default() }
}
//#endregion 🔖️Diff
