//! 🔺️ `change-dfm` sparse diff construction — writes only `En1997Diff.d_f_m` from the payload.

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::change_d_f_m::mutation::ChangeDFM;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeDFM, _base: &En1997Snapshot) -> En1997Diff {
    En1997Diff { d_f_m: Some(payload.new_d_f_m.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
