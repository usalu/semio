//! 🔺️ `change-silo-mu` — sparse diff construction.

use super::ChangeSiloMu;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeSiloMu, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_silo_mu.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Silo mu must be a finite number.", Vec::<String>::new());
    }
    if base.silo_mu == payload.new_silo_mu {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Silo mu already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { silo_mu: Some(payload.new_silo_mu.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
