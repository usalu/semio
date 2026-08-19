//! ↩️ `change-fire-resistance-min` inverse — restores the pre-change `fire_resistance_min` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1996::mutations::change_fire_resistance_min::mutation::ChangeFireResistanceMin;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeFireResistanceMin, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    vec![En1996Mutation::ChangeFireResistanceMin(ChangeFireResistanceMin { new_fire_resistance_min: base.fire_resistance_min.clone() })]
}
//#endregion 🔖️Inverse
