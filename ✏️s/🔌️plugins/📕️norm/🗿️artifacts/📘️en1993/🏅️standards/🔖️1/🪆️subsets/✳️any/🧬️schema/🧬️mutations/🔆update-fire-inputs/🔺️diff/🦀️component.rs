//! 🔺️ `update-fire-inputs` — sparse diff construction.

use super::mutation::UpdateFireInputs;
use crate::artifacts::en1993::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateFireInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_fire_thickness_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Fire thickness mm must be a finite number, got {}.", payload.new_fire_thickness_mm), Vec::<String>::new());
    }
    if !payload.new_fire_massivity.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Fire massivity must be a finite number, got {}.", payload.new_fire_massivity), Vec::<String>::new());
    }
    if !payload.new_fire_mu_0.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Fire mu 0 must be a finite number, got {}.", payload.new_fire_mu_0), Vec::<String>::new());
    }
    if !payload.new_fire_design_temperature_c.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Fire design temperature c must be a finite number, got {}.", payload.new_fire_design_temperature_c), Vec::<String>::new());
    }
    if base.fire_thickness_mm == payload.new_fire_thickness_mm && base.fire_rating == payload.new_fire_rating && base.fire_massivity == payload.new_fire_massivity && base.fire_mu_0 == payload.new_fire_mu_0 && base.fire_design_temperature_c == payload.new_fire_design_temperature_c {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "This facet already has these values.");
    }
    protocol::MutationOutcome::new(En1993Diff { fire_thickness_mm: Some(payload.new_fire_thickness_mm), fire_rating: Some(payload.new_fire_rating.clone()), fire_massivity: Some(payload.new_fire_massivity), fire_mu_0: Some(payload.new_fire_mu_0), fire_design_temperature_c: Some(payload.new_fire_design_temperature_c), ..Default::default() })
}
//#endregion 🔖️Diff
