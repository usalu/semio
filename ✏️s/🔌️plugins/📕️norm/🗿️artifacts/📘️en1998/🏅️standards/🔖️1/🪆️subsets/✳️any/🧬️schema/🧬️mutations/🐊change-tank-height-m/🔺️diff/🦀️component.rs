//! 🔺️ `change-tank-height-m` sparse diff construction — writes only `En1998Diff.tank_height_m` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_tank_height_m::mutation::ChangeTankHeightM;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeTankHeightM, _base: &En1998Snapshot) -> En1998Diff {
    En1998Diff { tank_height_m: Some(payload.new_tank_height_m.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
