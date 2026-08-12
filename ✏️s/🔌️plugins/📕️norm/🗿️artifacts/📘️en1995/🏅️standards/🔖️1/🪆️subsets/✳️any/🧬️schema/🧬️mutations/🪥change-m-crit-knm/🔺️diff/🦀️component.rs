//! 🔺️ `change-m-crit-knm` sparse diff construction — writes only `En1995Diff.m_crit_knm` from the payload.

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::change_m_crit_knm::mutation::ChangeMCritKnm;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeMCritKnm, _base: &En1995Snapshot) -> En1995Diff {
    En1995Diff { m_crit_knm: Some(payload.new_m_crit_knm.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
