//! 🔺️ `update-bridge-inputs` — sparse diff construction.

use super::UpdateBridgeInputs;
use crate::artifacts::en1993::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateBridgeInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_bridge_lambda.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Bridge lambda must be a finite number, got {}.", payload.new_bridge_lambda), Vec::<String>::new());
    }
    if !payload.new_bridge_phi_2.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Bridge phi 2 must be a finite number, got {}.", payload.new_bridge_phi_2), Vec::<String>::new());
    }
    if !payload.new_bridge_delta_sigma_p_mpa.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Bridge delta sigma p mpa must be a finite number, got {}.", payload.new_bridge_delta_sigma_p_mpa), Vec::<String>::new());
    }
    if base.bridge_lambda == payload.new_bridge_lambda && base.bridge_phi_2 == payload.new_bridge_phi_2 && base.bridge_delta_sigma_p_mpa == payload.new_bridge_delta_sigma_p_mpa {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "This facet already has these values.");
    }
    protocol::MutationOutcome::new(En1993Diff { bridge_lambda: Some(payload.new_bridge_lambda), bridge_phi_2: Some(payload.new_bridge_phi_2), bridge_delta_sigma_p_mpa: Some(payload.new_bridge_delta_sigma_p_mpa), ..Default::default() })
}
//#endregion 🔖️Diff
