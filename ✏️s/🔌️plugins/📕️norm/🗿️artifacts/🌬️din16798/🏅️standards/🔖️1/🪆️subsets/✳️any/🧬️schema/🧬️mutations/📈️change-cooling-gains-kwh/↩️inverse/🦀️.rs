//! ↩️ `change-cooling-gains-kwh` inverse — restores the pre-change `cooling_gains_kwh` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_cooling_gains_kwh::ChangeCoolingGainsKwh;
use crate::mutations::Din16798Mutation;
use crate::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeCoolingGainsKwh, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeCoolingGainsKwh(ChangeCoolingGainsKwh { new_cooling_gains_kwh: base.cooling_gains_kwh })]
}
//#endregion 🔖️Inverse
