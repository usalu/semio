//! 🔺️ `change-f-yk` sparse diff construction — writes only `En1992Diff.f_yk` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_f_yk::mutation::ChangeFYk;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeFYk, _base: &En1992Snapshot) -> En1992Diff {
    En1992Diff { f_yk: Some(payload.new_f_yk.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
