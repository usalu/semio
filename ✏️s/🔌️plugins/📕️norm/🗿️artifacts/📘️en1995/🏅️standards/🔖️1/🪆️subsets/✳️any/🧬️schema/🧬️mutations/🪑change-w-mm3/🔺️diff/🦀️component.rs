//! 🔺️ `change-w-mm3` sparse diff construction — writes only `En1995Diff.w_mm3` from the payload.

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::change_w_mm3::mutation::ChangeWMm3;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeWMm3, _base: &En1995Snapshot) -> En1995Diff {
    En1995Diff { w_mm3: Some(payload.new_w_mm3.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
