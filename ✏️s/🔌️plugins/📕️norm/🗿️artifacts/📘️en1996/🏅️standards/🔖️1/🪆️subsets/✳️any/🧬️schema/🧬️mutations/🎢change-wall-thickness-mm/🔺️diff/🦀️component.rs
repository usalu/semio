//! 🔺️ `change-wall-thickness-mm` sparse diff construction — writes only `En1996Diff.wall_thickness_mm` from the payload.

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::change_wall_thickness_mm::mutation::ChangeWallThicknessMm;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeWallThicknessMm, _base: &En1996Snapshot) -> En1996Diff {
    En1996Diff { wall_thickness_mm: Some(payload.new_wall_thickness_mm.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
