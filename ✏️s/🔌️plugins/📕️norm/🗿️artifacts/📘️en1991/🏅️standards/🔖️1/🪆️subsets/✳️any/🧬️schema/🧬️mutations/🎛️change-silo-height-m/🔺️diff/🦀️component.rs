//! 🔺️ `change-silo-height-m` — sparse diff construction.

use super::mutation::ChangeSiloHeightM;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ChangeSiloHeightM, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_silo_height_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Silo height m must be a finite number.", Vec::<String>::new());
    }
    if base.silo_height_m == payload.new_silo_height_m {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Silo height m already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { silo_height_m: Some(payload.new_silo_height_m.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
