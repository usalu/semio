//! 🔺️ `change-bm` sparse diff construction — writes only `En1997Diff.b_m` from the payload.

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::change_b_m::mutation::ChangeBM;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeBM, _base: &En1997Snapshot) -> En1997Diff {
    En1997Diff { b_m: Some(payload.new_b_m.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
