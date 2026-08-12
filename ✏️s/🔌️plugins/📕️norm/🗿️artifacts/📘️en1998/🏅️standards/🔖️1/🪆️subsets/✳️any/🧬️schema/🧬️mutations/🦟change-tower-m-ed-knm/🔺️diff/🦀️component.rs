//! 🔺️ `change-tower-m-ed-knm` sparse diff construction — writes only `En1998Diff.tower_m_ed_knm` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_tower_m_ed_knm::mutation::ChangeTowerMEdKnm;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeTowerMEdKnm, _base: &En1998Snapshot) -> En1998Diff {
    En1998Diff { tower_m_ed_knm: Some(payload.new_tower_m_ed_knm.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
