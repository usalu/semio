//! ↩️ `change-fire-resistance-min` — undo restores BASE's fire resistance.

use super::mutation::ChangeFireResistanceMin;
use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeFireResistanceMin, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeFireResistanceMin(ChangeFireResistanceMin { new_fire_resistance_min: base.fire_resistance_min.clone() })]
}
//#endregion 🔖️Inverse
