//! 🔺️ `change-m-ed-knm` sparse diff construction — writes only `En1996Diff.m_ed_knm` from the payload.

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::change_m_ed_knm::mutation::ChangeMEdKnm;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeMEdKnm, _base: &En1996Snapshot) -> En1996Diff {
    En1996Diff { m_ed_knm: Some(payload.new_m_ed_knm.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
