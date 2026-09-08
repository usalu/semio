//! 🔺️ `change-bridge-delta-sigma-s-mpa` sparse diff construction — writes only `En1992Diff.bridge_delta_sigma_s_mpa` from the payload.

use crate::diff::En1992Diff;
use crate::mutations::change_bridge_delta_sigma_s_mpa::ChangeBridgeDeltaSigmaSMpa;
use crate::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeBridgeDeltaSigmaSMpa, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_bridge_delta_sigma_s_mpa.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Bridge delta sigma s mpa must be a finite number.", Vec::<String>::new());
    }
    if base.bridge_delta_sigma_s_mpa == payload.new_bridge_delta_sigma_s_mpa {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Bridge delta sigma s mpa already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { bridge_delta_sigma_s_mpa: Some(payload.new_bridge_delta_sigma_s_mpa), ..Default::default() })
}
//#endregion 🔖️Diff
