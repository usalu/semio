//! 🔺️ `change-shear-area-mm2` sparse diff construction — writes only `En1996Diff.shear_area_mm2` from the payload.

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::change_shear_area_mm2::mutation::ChangeShearAreaMm2;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeShearAreaMm2, _base: &En1996Snapshot) -> En1996Diff {
    En1996Diff { shear_area_mm2: Some(payload.new_shear_area_mm2.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
