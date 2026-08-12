//! 🔺️ `change-as-mm2` sparse diff construction — writes only `En1992Diff.a_s_mm2` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_a_s_mm2::mutation::ChangeASMm2;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeASMm2, _base: &En1992Snapshot) -> En1992Diff {
    En1992Diff { a_s_mm2: Some(payload.new_a_s_mm2.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
