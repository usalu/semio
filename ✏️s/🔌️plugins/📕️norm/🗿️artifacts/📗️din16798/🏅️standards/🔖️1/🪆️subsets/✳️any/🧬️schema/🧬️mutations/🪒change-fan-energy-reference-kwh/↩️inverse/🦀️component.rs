//! ↩️ `change-fan-energy-reference-kwh` inverse — restores the pre-change `fan_energy_reference_kwh` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din16798::mutations::change_fan_energy_reference_kwh::mutation::ChangeFanEnergyReferenceKwh;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeFanEnergyReferenceKwh, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeFanEnergyReferenceKwh(ChangeFanEnergyReferenceKwh { new_fan_energy_reference_kwh: base.fan_energy_reference_kwh.clone() })]
}
//#endregion 🔖️Inverse
