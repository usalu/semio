//! 🔺️ `change-cooling-utilization-factor` sparse diff construction — writes only `Din16798Diff.cooling_utilization_factor` from the payload.

use crate::diff::Din16798Diff;
use crate::mutations::change_cooling_utilization_factor::ChangeCoolingUtilizationFactor;
use crate::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeCoolingUtilizationFactor, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_cooling_utilization_factor.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Cooling gain utilization factor must be a finite number, got {}.", payload.new_cooling_utilization_factor), Vec::<String>::new());
    }
    if base.cooling_utilization_factor == payload.new_cooling_utilization_factor {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Cooling gain utilization factor is already {}.", payload.new_cooling_utilization_factor));
    }
    protocol::MutationOutcome::new(Din16798Diff { cooling_utilization_factor: Some(payload.new_cooling_utilization_factor), ..Default::default() })
}
//#endregion 🔖️Diff
