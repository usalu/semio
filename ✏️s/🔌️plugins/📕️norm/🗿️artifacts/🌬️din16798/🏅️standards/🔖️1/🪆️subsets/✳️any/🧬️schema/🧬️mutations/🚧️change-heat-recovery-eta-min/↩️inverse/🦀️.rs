//! ↩️ `change-heat-recovery-eta-min` inverse — restores the pre-change `heat_recovery_eta_min` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_heat_recovery_eta_min::ChangeHeatRecoveryEtaMin;
use crate::mutations::Din16798Mutation;
use crate::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeHeatRecoveryEtaMin, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeHeatRecoveryEtaMin(ChangeHeatRecoveryEtaMin { new_heat_recovery_eta_min: base.heat_recovery_eta_min })]
}
//#endregion 🔖️Inverse
