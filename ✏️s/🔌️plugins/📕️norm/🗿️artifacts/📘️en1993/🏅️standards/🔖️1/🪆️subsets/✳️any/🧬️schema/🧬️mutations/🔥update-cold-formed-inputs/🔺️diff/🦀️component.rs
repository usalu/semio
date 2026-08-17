//! 🔺️ `update-cold-formed-inputs` — sparse diff construction.

use super::mutation::UpdateColdFormedInputs;
use crate::artifacts::en1993::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateColdFormedInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_cf_b_bar_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Cf b bar mm must be a finite number, got {}.", payload.new_cf_b_bar_mm), Vec::<String>::new());
    }
    if !payload.new_cf_t_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Cf t mm must be a finite number, got {}.", payload.new_cf_t_mm), Vec::<String>::new());
    }
    if !payload.new_cf_k_sigma.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Cf k sigma must be a finite number, got {}.", payload.new_cf_k_sigma), Vec::<String>::new());
    }
    if !payload.new_cf_psi.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Cf psi must be a finite number, got {}.", payload.new_cf_psi), Vec::<String>::new());
    }
    if !payload.new_cf_n_ed_kn.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Cf n ed kn must be a finite number, got {}.", payload.new_cf_n_ed_kn), Vec::<String>::new());
    }
    if !payload.new_cf_gross_resistance_kn.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Cf gross resistance kn must be a finite number, got {}.", payload.new_cf_gross_resistance_kn), Vec::<String>::new());
    }
    if base.cf_b_bar_mm == payload.new_cf_b_bar_mm && base.cf_t_mm == payload.new_cf_t_mm && base.cf_k_sigma == payload.new_cf_k_sigma && base.cf_psi == payload.new_cf_psi && base.cf_n_ed_kn == payload.new_cf_n_ed_kn && base.cf_gross_resistance_kn == payload.new_cf_gross_resistance_kn {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "This facet already has these values.");
    }
    protocol::MutationOutcome::new(En1993Diff { cf_b_bar_mm: Some(payload.new_cf_b_bar_mm), cf_t_mm: Some(payload.new_cf_t_mm), cf_k_sigma: Some(payload.new_cf_k_sigma), cf_psi: Some(payload.new_cf_psi), cf_n_ed_kn: Some(payload.new_cf_n_ed_kn), cf_gross_resistance_kn: Some(payload.new_cf_gross_resistance_kn), ..Default::default() })
}
//#endregion 🔖️Diff
