//! 🔺️ `update-tower-inputs` — sparse diff construction.

use super::UpdateTowerInputs;
use crate::artifacts::en1993::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateTowerInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_tower_wind_factor.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Tower wind factor must be a finite number, got {}.", payload.new_tower_wind_factor), Vec::<String>::new());
    }
    if !payload.new_tower_n_ed_kn.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Tower n ed kn must be a finite number, got {}.", payload.new_tower_n_ed_kn), Vec::<String>::new());
    }
    if base.tower_wind_factor == payload.new_tower_wind_factor && base.tower_n_ed_kn == payload.new_tower_n_ed_kn {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "This facet already has these values.");
    }
    protocol::MutationOutcome::new(En1993Diff { tower_wind_factor: Some(payload.new_tower_wind_factor), tower_n_ed_kn: Some(payload.new_tower_n_ed_kn), ..Default::default() })
}
//#endregion 🔖️Diff
