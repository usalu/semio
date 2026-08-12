//! 🔺️ `change-wall-r` sparse diff construction — writes only `En1998Diff.wall_r` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_wall_r::mutation::ChangeWallR;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeWallR, _base: &En1998Snapshot) -> En1998Diff {
    En1998Diff { wall_r: Some(payload.new_wall_r.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
