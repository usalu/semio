//! ↩️ `change-energy-carrier` inverse — restores the pre-change `energy_carrier` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_energy_carrier::ChangeEnergyCarrier;
use crate::mutations::Din18599Mutation;
use crate::Din18599Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeEnergyCarrier, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
    vec![Din18599Mutation::ChangeEnergyCarrier(ChangeEnergyCarrier { new_energy_carrier: base.energy_carrier.clone() })]
}
//#endregion 🔖️Inverse
