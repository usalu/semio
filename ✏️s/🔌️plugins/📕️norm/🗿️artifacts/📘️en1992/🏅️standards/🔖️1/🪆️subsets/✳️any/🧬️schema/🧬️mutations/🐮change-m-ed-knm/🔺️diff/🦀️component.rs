//! 🔺️ `change-m-ed-knm` sparse diff construction — writes only `En1992Diff.m_ed_knm` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_m_ed_knm::mutation::ChangeMEdKnm;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeMEdKnm, _base: &En1992Snapshot) -> En1992Diff {
    En1992Diff { m_ed_knm: Some(payload.new_m_ed_knm.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
