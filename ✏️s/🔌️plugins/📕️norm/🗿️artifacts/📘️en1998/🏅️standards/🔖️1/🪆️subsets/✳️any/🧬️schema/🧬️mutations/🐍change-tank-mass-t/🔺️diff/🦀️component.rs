//! 🔺️ `change-tank-mass-t` sparse diff construction — writes only `En1998Diff.tank_mass_t` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_tank_mass_t::mutation::ChangeTankMassT;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeTankMassT, _base: &En1998Snapshot) -> En1998Diff {
    En1998Diff { tank_mass_t: Some(payload.new_tank_mass_t.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
