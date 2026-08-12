//! 🔺️ `change-en-ground-type` sparse diff construction — writes only `En1998Diff.en_ground_type` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_en_ground_type::mutation::ChangeEnGroundType;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeEnGroundType, _base: &En1998Snapshot) -> En1998Diff {
    En1998Diff { en_ground_type: Some(payload.new_en_ground_type.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
