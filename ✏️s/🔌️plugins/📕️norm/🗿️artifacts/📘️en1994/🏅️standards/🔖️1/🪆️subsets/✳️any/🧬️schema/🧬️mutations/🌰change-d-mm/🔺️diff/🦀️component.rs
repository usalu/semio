//! 🔺️ `change-d-mm` — sparse diff construction.

use super::mutation::ChangeDMm;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ChangeDMm, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if !payload.new_d_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "D mm must be a finite number.", Vec::<String>::new());
    }
    if base.d_mm == payload.new_d_mm {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "D mm already has this value.");
    }
    protocol::MutationOutcome::new(En1994Diff { d_mm: Some(payload.new_d_mm.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
