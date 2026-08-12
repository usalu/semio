//! 🔺️ `change-foundation-area-m2` sparse diff construction — writes only `En1998Diff.foundation_area_m2` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_foundation_area_m2::mutation::ChangeFoundationAreaM2;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeFoundationAreaM2, _base: &En1998Snapshot) -> En1998Diff {
    En1998Diff { foundation_area_m2: Some(payload.new_foundation_area_m2.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
