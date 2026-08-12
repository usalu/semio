//! 🔺️ `change-area-mm2` sparse diff construction — writes only `En1996Diff.area_mm2` from the payload.

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::change_area_mm2::mutation::ChangeAreaMm2;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAreaMm2, _base: &En1996Snapshot) -> En1996Diff {
    En1996Diff { area_mm2: Some(payload.new_area_mm2.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
