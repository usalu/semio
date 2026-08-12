//! 🔺️ `change-bearing-d-ed-mm` sparse diff construction — writes only `En1998Diff.bearing_d_ed_mm` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_bearing_d_ed_mm::mutation::ChangeBearingDEdMm;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeBearingDEdMm, _base: &En1998Snapshot) -> En1998Diff {
    En1998Diff { bearing_d_ed_mm: Some(payload.new_bearing_d_ed_mm.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
