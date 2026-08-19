//! 🔺️ `change-silo-k` — sparse diff construction.

use super::mutation::ChangeSiloK;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ChangeSiloK, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_silo_k.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Silo k must be a finite number.", Vec::<String>::new());
    }
    if base.silo_k == payload.new_silo_k {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Silo k already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { silo_k: Some(payload.new_silo_k.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
