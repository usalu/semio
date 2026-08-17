//! 🔺️ `change-bridge-moment-resistance-knm` — sparse diff construction.

use super::mutation::ChangeBridgeMomentResistanceKnm;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeBridgeMomentResistanceKnm, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_bridge_moment_resistance_knm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Bridge moment resistance knm must be a finite number.", Vec::<String>::new());
    }
    if base.bridge_moment_resistance_knm == payload.new_bridge_moment_resistance_knm {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Bridge moment resistance knm already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { bridge_moment_resistance_knm: Some(payload.new_bridge_moment_resistance_knm.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
