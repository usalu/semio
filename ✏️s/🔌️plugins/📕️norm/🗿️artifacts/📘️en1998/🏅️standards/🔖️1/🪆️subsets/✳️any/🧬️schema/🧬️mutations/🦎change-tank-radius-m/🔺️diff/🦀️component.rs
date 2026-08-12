//! 🔺️ `change-tank-radius-m` sparse diff construction — writes only `En1998Diff.tank_radius_m` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_tank_radius_m::mutation::ChangeTankRadiusM;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeTankRadiusM, _base: &En1998Snapshot) -> En1998Diff {
    En1998Diff { tank_radius_m: Some(payload.new_tank_radius_m.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
