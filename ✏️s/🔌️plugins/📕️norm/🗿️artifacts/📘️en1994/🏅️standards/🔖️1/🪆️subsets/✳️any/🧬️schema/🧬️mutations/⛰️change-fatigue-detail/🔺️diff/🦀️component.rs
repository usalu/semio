//! 🔺️ `change-fatigue-detail` — sparse diff construction.

use super::mutation::ChangeFatigueDetail;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeFatigueDetail, _base: &En1994Snapshot) -> En1994Diff {
    En1994Diff { fatigue_detail: Some(payload.new_fatigue_detail.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
