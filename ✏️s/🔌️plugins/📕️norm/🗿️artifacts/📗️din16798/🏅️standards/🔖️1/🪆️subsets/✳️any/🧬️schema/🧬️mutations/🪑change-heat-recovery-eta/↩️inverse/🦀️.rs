//! ↩️ `change-heat-recovery-eta` inverse — restores the pre-change `heat_recovery_eta` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din16798::mutations::change_heat_recovery_eta::ChangeHeatRecoveryEta;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeHeatRecoveryEta, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeHeatRecoveryEta(ChangeHeatRecoveryEta { new_heat_recovery_eta: base.heat_recovery_eta.clone() })]
}
//#endregion 🔖️Inverse
