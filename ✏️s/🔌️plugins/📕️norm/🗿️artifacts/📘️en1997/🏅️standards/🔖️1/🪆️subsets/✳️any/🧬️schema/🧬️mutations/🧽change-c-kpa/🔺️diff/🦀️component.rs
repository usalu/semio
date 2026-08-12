//! 🔺️ `change-c-kpa` sparse diff construction — writes only `En1997Diff.c_kpa` from the payload.

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::change_c_kpa::mutation::ChangeCKpa;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeCKpa, _base: &En1997Snapshot) -> En1997Diff {
    En1997Diff { c_kpa: Some(payload.new_c_kpa.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
