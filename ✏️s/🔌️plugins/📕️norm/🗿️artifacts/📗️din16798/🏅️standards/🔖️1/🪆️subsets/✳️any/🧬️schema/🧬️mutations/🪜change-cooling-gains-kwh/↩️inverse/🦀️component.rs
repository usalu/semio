//! ↩️ `change-cooling-gains-kwh` inverse — restores the pre-change `cooling_gains_kwh` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din16798::mutations::change_cooling_gains_kwh::mutation::ChangeCoolingGainsKwh;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeCoolingGainsKwh, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeCoolingGainsKwh(ChangeCoolingGainsKwh { new_cooling_gains_kwh: base.cooling_gains_kwh.clone() })]
}
//#endregion 🔖️Inverse
