//! 🔺️ `change-bearing-d-rd-mm` sparse diff construction — writes only `En1998Diff.bearing_d_rd_mm` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_bearing_d_rd_mm::mutation::ChangeBearingDRdMm;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeBearingDRdMm, _base: &En1998Snapshot) -> En1998Diff {
    En1998Diff { bearing_d_rd_mm: Some(payload.new_bearing_d_rd_mm.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
