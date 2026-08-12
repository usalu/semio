//! 🔺️ `change-heat-recovery-eta-min` sparse diff construction — writes only `Din16798Diff.heat_recovery_eta_min` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_heat_recovery_eta_min::mutation::ChangeHeatRecoveryEtaMin;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeHeatRecoveryEtaMin, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { heat_recovery_eta_min: Some(payload.new_heat_recovery_eta_min.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
