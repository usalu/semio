//! 🔺️ `update-tension-component-inputs` — sparse diff construction.

use super::mutation::UpdateTensionComponentInputs;
use crate::artifacts::en1993::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateTensionComponentInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_tension_component_f_uk_kn.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Tension component f uk kn must be a finite number, got {}.", payload.new_tension_component_f_uk_kn), Vec::<String>::new());
    }
    if !payload.new_tension_component_f_k_kn.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Tension component f k kn must be a finite number, got {}.", payload.new_tension_component_f_k_kn), Vec::<String>::new());
    }
    if !payload.new_tension_component_n_ed_kn.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Tension component n ed kn must be a finite number, got {}.", payload.new_tension_component_n_ed_kn), Vec::<String>::new());
    }
    if base.tension_component_f_uk_kn == payload.new_tension_component_f_uk_kn && base.tension_component_f_k_kn == payload.new_tension_component_f_k_kn && base.tension_component_n_ed_kn == payload.new_tension_component_n_ed_kn {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "This facet already has these values.");
    }
    protocol::MutationOutcome::new(En1993Diff { tension_component_f_uk_kn: Some(payload.new_tension_component_f_uk_kn), tension_component_f_k_kn: Some(payload.new_tension_component_f_k_kn), tension_component_n_ed_kn: Some(payload.new_tension_component_n_ed_kn), ..Default::default() })
}
//#endregion 🔖️Diff
