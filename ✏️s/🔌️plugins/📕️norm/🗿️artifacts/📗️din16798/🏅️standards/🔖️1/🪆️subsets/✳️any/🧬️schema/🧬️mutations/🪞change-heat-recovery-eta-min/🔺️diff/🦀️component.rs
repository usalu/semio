//! 🔺️ `change-heat-recovery-eta-min` sparse diff construction — writes only `Din16798Diff.heat_recovery_eta_min` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_heat_recovery_eta_min::mutation::ChangeHeatRecoveryEtaMin;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeHeatRecoveryEtaMin, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_heat_recovery_eta_min.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Minimum heat recovery efficiency must be a finite number, got {}.", payload.new_heat_recovery_eta_min), Vec::<String>::new());
    }
    if base.heat_recovery_eta_min == payload.new_heat_recovery_eta_min {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Minimum heat recovery efficiency is already {}.", payload.new_heat_recovery_eta_min));
    }
    protocol::MutationOutcome::new(Din16798Diff { heat_recovery_eta_min: Some(payload.new_heat_recovery_eta_min.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
