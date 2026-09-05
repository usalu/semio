//! ↩️ `change-wind-zone` — undo restores BASE's wind zone.

use super::ChangeWindZone;
use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeWindZone, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeWindZone(ChangeWindZone { new_wind_zone: base.wind_zone.clone() })]
}
//#endregion 🔖️Inverse
