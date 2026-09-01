//! ↩️ `change-night-setback-k` inverse — restores the pre-change `night_setback_k` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din16798::mutations::change_night_setback_k::ChangeNightSetbackK;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeNightSetbackK, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeNightSetbackK(ChangeNightSetbackK { new_night_setback_k: base.night_setback_k.clone() })]
}
//#endregion 🔖️Inverse
