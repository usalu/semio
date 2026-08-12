//! 🔺️ `change-fire-resistance-min` sparse diff construction — writes only `En1996Diff.fire_resistance_min` from the payload.

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::change_fire_resistance_min::mutation::ChangeFireResistanceMin;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeFireResistanceMin, _base: &En1996Snapshot) -> En1996Diff {
    En1996Diff { fire_resistance_min: Some(payload.new_fire_resistance_min.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
