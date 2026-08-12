//! 🔺️ `change-pile-base-area-m2` sparse diff construction — writes only `En1997Diff.pile_base_area_m2` from the payload.

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::change_pile_base_area_m2::mutation::ChangePileBaseAreaM2;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangePileBaseAreaM2, _base: &En1997Snapshot) -> En1997Diff {
    En1997Diff { pile_base_area_m2: Some(payload.new_pile_base_area_m2.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
