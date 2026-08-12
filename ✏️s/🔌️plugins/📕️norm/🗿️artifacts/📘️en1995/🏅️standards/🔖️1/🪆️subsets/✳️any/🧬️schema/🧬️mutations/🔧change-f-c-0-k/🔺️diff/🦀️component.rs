//! 🔺️ `change-f-c-0-k` sparse diff construction — writes only `En1995Diff.f_c_0_k` from the payload.

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::change_f_c_0_k::mutation::ChangeFC0K;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeFC0K, _base: &En1995Snapshot) -> En1995Diff {
    En1995Diff { f_c_0_k: Some(payload.new_f_c_0_k.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
