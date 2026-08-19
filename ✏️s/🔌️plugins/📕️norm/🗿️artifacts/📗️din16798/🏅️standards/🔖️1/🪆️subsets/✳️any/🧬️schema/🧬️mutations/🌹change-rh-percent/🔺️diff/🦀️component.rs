//! 🔺️ `change-rh-percent` sparse diff construction — writes only `Din16798Diff.rh_percent` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_rh_percent::mutation::ChangeRhPercent;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeRhPercent, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_rh_percent.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Relative humidity must be a finite number, got {}.", payload.new_rh_percent), Vec::<String>::new());
    }
    if payload.new_rh_percent < 0.0 || payload.new_rh_percent > 100.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Relative humidity must be between 0 and 100 percent, got {}.", payload.new_rh_percent), Vec::<String>::new());
    }
    if base.rh_percent == payload.new_rh_percent {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Relative humidity is already {}.", payload.new_rh_percent));
    }
    protocol::MutationOutcome::new(Din16798Diff { rh_percent: Some(payload.new_rh_percent.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
