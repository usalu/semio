//! 🔺️ `update-pile-inputs` — sparse diff construction.

use super::UpdatePileInputs;
use crate::artifacts::en1993::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdatePileInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_pile_sigma_mpa.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Pile sigma mpa must be a finite number, got {}.", payload.new_pile_sigma_mpa), Vec::<String>::new());
    }
    if !payload.new_pile_k_red.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Pile k red must be a finite number, got {}.", payload.new_pile_k_red), Vec::<String>::new());
    }
    if !payload.new_pile_n_ed_kn.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Pile n ed kn must be a finite number, got {}.", payload.new_pile_n_ed_kn), Vec::<String>::new());
    }
    if base.pile_sigma_mpa == payload.new_pile_sigma_mpa && base.pile_k_red == payload.new_pile_k_red && base.pile_n_ed_kn == payload.new_pile_n_ed_kn {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "This facet already has these values.");
    }
    protocol::MutationOutcome::new(En1993Diff { pile_sigma_mpa: Some(payload.new_pile_sigma_mpa), pile_k_red: Some(payload.new_pile_k_red), pile_n_ed_kn: Some(payload.new_pile_n_ed_kn), ..Default::default() })
}
//#endregion 🔖️Diff
