//! ↩️ `change-fire-resistance-min` inverse — restores the pre-change `fire_resistance_min` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_fire_resistance_min::ChangeFireResistanceMin;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeFireResistanceMin, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    vec![En1996Mutation::ChangeFireResistanceMin(ChangeFireResistanceMin { new_fire_resistance_min: base.fire_resistance_min })]
}
//#endregion 🔖️Inverse
