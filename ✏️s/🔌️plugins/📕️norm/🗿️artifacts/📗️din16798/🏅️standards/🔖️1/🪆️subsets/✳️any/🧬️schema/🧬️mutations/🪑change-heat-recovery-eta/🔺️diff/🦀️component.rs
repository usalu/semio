//! 🔺️ `change-heat-recovery-eta` sparse diff construction — writes only `Din16798Diff.heat_recovery_eta` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_heat_recovery_eta::mutation::ChangeHeatRecoveryEta;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeHeatRecoveryEta, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_heat_recovery_eta.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Heat recovery efficiency must be a finite number, got {}.", payload.new_heat_recovery_eta), Vec::<String>::new());
    }
    if base.heat_recovery_eta == payload.new_heat_recovery_eta {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Heat recovery efficiency is already {}.", payload.new_heat_recovery_eta));
    }
    protocol::MutationOutcome::new(Din16798Diff { heat_recovery_eta: Some(payload.new_heat_recovery_eta.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
