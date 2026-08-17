//! 🔺️ `change-delta-tk` — sparse diff construction.

use super::mutation::ChangeDeltaTK;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeDeltaTK, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_delta_t_k.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Delta tk must be a finite number.", Vec::<String>::new());
    }
    if base.delta_t_k == payload.new_delta_t_k {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Delta tk already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { delta_t_k: Some(payload.new_delta_t_k.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
