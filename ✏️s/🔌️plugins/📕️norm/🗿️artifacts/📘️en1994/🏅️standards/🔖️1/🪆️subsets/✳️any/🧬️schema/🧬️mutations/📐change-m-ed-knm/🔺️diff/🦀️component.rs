//! 🔺️ `change-m-ed-knm` — sparse diff construction.

use super::mutation::ChangeMEdKnm;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeMEdKnm, _base: &En1994Snapshot) -> En1994Diff {
    En1994Diff { m_ed_knm: Some(payload.new_m_ed_knm), ..Default::default() }
}
//#endregion 🔖️Diff
