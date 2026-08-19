//! ↩️ `change-cooling-utilization-factor` inverse — restores the pre-change `cooling_utilization_factor` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din16798::mutations::change_cooling_utilization_factor::mutation::ChangeCoolingUtilizationFactor;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeCoolingUtilizationFactor, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeCoolingUtilizationFactor(ChangeCoolingUtilizationFactor { new_cooling_utilization_factor: base.cooling_utilization_factor.clone() })]
}
//#endregion 🔖️Inverse
