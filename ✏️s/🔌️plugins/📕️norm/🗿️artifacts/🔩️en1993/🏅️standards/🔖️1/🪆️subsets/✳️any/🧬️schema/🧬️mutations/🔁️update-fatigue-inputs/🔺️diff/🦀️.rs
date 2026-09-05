//! 🔺️ `update-fatigue-inputs` — sparse diff construction.

use super::UpdateFatigueInputs;
use crate::artifacts::en1993::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateFatigueInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_delta_sigma_mpa.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Delta sigma mpa must be a finite number, got {}.", payload.new_delta_sigma_mpa), Vec::<String>::new());
    }
    if base.delta_sigma_mpa == payload.new_delta_sigma_mpa && base.fatigue_category == payload.new_fatigue_category && base.fatigue_method == payload.new_fatigue_method {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "This facet already has these values.");
    }
    protocol::MutationOutcome::new(En1993Diff { delta_sigma_mpa: Some(payload.new_delta_sigma_mpa), fatigue_category: Some(payload.new_fatigue_category), fatigue_method: Some(payload.new_fatigue_method.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
