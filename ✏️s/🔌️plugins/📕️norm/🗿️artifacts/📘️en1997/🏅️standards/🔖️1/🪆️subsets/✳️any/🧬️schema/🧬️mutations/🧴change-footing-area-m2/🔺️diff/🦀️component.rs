//! 🔺️ `change-footing-area-m2` sparse diff construction — writes only `En1997Diff.footing_area_m2` from the payload.

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::change_footing_area_m2::mutation::ChangeFootingAreaM2;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeFootingAreaM2, _base: &En1997Snapshot) -> En1997Diff {
    En1997Diff { footing_area_m2: Some(payload.new_footing_area_m2.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
