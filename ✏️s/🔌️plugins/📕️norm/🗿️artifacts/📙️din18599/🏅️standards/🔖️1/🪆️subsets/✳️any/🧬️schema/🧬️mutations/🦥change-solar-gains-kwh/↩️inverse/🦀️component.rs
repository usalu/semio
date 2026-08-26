//! ↩️ `change-solar-gains-kwh` inverse — restores the pre-change `solar_gains_kwh` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din18599::mutations::change_solar_gains_kwh::mutation::ChangeSolarGainsKwh;
use crate::artifacts::din18599::mutations::Din18599Mutation;
use crate::artifacts::din18599::Din18599Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeSolarGainsKwh, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
    vec![Din18599Mutation::ChangeSolarGainsKwh(ChangeSolarGainsKwh { new_solar_gains_kwh: base.solar_gains_kwh.clone() })]
}
//#endregion 🔖️Inverse
