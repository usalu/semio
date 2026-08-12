//! 🔺️ `change-f-v-k` sparse diff construction — writes only `En1995Diff.f_v_k` from the payload.

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::change_f_v_k::mutation::ChangeFVK;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeFVK, _base: &En1995Snapshot) -> En1995Diff {
    En1995Diff { f_v_k: Some(payload.new_f_v_k.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
