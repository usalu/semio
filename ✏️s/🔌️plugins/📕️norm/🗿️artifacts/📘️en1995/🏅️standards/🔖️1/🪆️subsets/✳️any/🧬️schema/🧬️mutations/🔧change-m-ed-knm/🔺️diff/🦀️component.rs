//! 🔺️ `change-m-ed-knm` sparse diff construction — writes only `En1995Diff.m_ed_knm` from the payload.

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::change_m_ed_knm::mutation::ChangeMEdKnm;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeMEdKnm, _base: &En1995Snapshot) -> En1995Diff {
    En1995Diff { m_ed_knm: Some(payload.new_m_ed_knm.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
