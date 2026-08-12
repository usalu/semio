//! 🔺️ `change-heat-recovery-eta` sparse diff construction — writes only `Din16798Diff.heat_recovery_eta` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_heat_recovery_eta::mutation::ChangeHeatRecoveryEta;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeHeatRecoveryEta, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { heat_recovery_eta: Some(payload.new_heat_recovery_eta.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
