//! 🔺️ `change-m-ed-knm` sparse diff construction — writes only `En1999Diff.m_ed_knm` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_m_ed_knm::mutation::ChangeMEdKnm;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeMEdKnm, _base: &En1999Snapshot) -> En1999Diff {
    En1999Diff { m_ed_knm: Some(payload.new_m_ed_knm.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
