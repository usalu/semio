//! ↩️ `change-co2-ppm` inverse — restores the pre-change `co2_ppm` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_co2_ppm::ChangeCo2Ppm;
use crate::mutations::Din16798Mutation;
use crate::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeCo2Ppm, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeCo2Ppm(ChangeCo2Ppm { new_co2_ppm: base.co2_ppm })]
}
//#endregion 🔖️Inverse
