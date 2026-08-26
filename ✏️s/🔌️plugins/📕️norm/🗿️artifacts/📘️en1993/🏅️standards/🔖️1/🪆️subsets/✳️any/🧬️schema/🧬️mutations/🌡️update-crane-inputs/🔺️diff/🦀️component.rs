//! 🔺️ `update-crane-inputs` — sparse diff construction.

use super::mutation::UpdateCraneInputs;
use crate::artifacts::en1993::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateCraneInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_crane_f_z_ed_kn.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Crane f z ed kn must be a finite number, got {}.", payload.new_crane_f_z_ed_kn), Vec::<String>::new());
    }
    if !payload.new_crane_wheel_contact_length_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Crane wheel contact length mm must be a finite number, got {}.", payload.new_crane_wheel_contact_length_mm), Vec::<String>::new());
    }
    if !payload.new_crane_dispersion_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Crane dispersion mm must be a finite number, got {}.", payload.new_crane_dispersion_mm), Vec::<String>::new());
    }
    if !payload.new_crane_t_w_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Crane t w mm must be a finite number, got {}.", payload.new_crane_t_w_mm), Vec::<String>::new());
    }
    if base.crane_f_z_ed_kn == payload.new_crane_f_z_ed_kn
        && base.crane_wheel_contact_length_mm == payload.new_crane_wheel_contact_length_mm
        && base.crane_dispersion_mm == payload.new_crane_dispersion_mm
        && base.crane_t_w_mm == payload.new_crane_t_w_mm
    {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "This facet already has these values.");
    }
    protocol::MutationOutcome::new(En1993Diff {
        crane_f_z_ed_kn: Some(payload.new_crane_f_z_ed_kn),
        crane_wheel_contact_length_mm: Some(payload.new_crane_wheel_contact_length_mm),
        crane_dispersion_mm: Some(payload.new_crane_dispersion_mm),
        crane_t_w_mm: Some(payload.new_crane_t_w_mm),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
