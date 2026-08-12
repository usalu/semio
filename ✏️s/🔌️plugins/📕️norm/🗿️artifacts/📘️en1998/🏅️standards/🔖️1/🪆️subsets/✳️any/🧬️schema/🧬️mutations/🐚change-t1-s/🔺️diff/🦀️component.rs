//! 🔺️ `change-t1-s` sparse diff construction — writes only `En1998Diff.t1_s` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_t1_s::mutation::ChangeT1S;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeT1S, _base: &En1998Snapshot) -> En1998Diff {
    En1998Diff { t1_s: Some(payload.new_t1_s.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
