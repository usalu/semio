//! 🔺️ `change-tower-m-rd-knm` sparse diff construction — writes only `En1998Diff.tower_m_rd_knm` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_tower_m_rd_knm::mutation::ChangeTowerMRdKnm;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeTowerMRdKnm, _base: &En1998Snapshot) -> En1998Diff {
    En1998Diff { tower_m_rd_knm: Some(payload.new_tower_m_rd_knm.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
