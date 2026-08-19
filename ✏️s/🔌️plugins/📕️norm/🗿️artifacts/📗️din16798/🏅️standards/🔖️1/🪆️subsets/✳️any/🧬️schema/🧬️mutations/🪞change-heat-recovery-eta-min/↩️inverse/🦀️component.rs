//! ↩️ `change-heat-recovery-eta-min` inverse — restores the pre-change `heat_recovery_eta_min` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din16798::mutations::change_heat_recovery_eta_min::mutation::ChangeHeatRecoveryEtaMin;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeHeatRecoveryEtaMin, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeHeatRecoveryEtaMin(ChangeHeatRecoveryEtaMin { new_heat_recovery_eta_min: base.heat_recovery_eta_min.clone() })]
}
//#endregion 🔖️Inverse
