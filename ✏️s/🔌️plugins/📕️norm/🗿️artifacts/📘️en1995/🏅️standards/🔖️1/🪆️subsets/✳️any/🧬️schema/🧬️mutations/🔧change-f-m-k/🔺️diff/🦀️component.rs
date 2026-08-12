//! 🔺️ `change-f-m-k` sparse diff construction — writes only `En1995Diff.f_m_k` from the payload.

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::change_f_m_k::mutation::ChangeFMK;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeFMK, _base: &En1995Snapshot) -> En1995Diff {
    En1995Diff { f_m_k: Some(payload.new_f_m_k.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
