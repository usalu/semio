//! 🔺️ `update-plated-inputs` — sparse diff construction.

use super::UpdatePlatedInputs;
use crate::artifacts::en1993::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdatePlatedInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_plated_lambda_p.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Plated lambda p must be a finite number, got {}.", payload.new_plated_lambda_p), Vec::<String>::new());
    }
    if !payload.new_plated_sigma_ed_mpa.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Plated sigma ed mpa must be a finite number, got {}.", payload.new_plated_sigma_ed_mpa), Vec::<String>::new());
    }
    if base.plated_lambda_p == payload.new_plated_lambda_p && base.plated_sigma_ed_mpa == payload.new_plated_sigma_ed_mpa {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "This facet already has these values.");
    }
    protocol::MutationOutcome::new(En1993Diff { plated_lambda_p: Some(payload.new_plated_lambda_p), plated_sigma_ed_mpa: Some(payload.new_plated_sigma_ed_mpa), ..Default::default() })
}
//#endregion 🔖️Diff
