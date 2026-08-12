//! 🔺️ `change-f-ck` sparse diff construction — writes only `En1992Diff.f_ck` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_f_ck::mutation::ChangeFCk;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeFCk, _base: &En1992Snapshot) -> En1992Diff {
    En1992Diff { f_ck: Some(payload.new_f_ck.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
