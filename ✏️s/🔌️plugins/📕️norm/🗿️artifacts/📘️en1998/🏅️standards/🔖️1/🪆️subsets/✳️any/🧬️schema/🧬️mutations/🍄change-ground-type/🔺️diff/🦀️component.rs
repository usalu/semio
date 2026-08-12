//! 🔺️ `change-ground-type` sparse diff construction — writes only `En1998Diff.ground_type` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_ground_type::mutation::ChangeGroundType;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeGroundType, _base: &En1998Snapshot) -> En1998Diff {
    En1998Diff { ground_type: Some(payload.new_ground_type.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
