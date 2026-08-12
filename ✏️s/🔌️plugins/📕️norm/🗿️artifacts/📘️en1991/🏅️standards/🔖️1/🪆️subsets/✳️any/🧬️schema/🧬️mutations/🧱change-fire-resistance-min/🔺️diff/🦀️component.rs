//! 🔺️ `change-fire-resistance-min` — sparse diff construction.

use super::mutation::ChangeFireResistanceMin;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeFireResistanceMin, _base: &En1991Snapshot) -> En1991Diff {
    En1991Diff { fire_resistance_min: Some(payload.new_fire_resistance_min.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
