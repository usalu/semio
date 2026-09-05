//! 🔺️ `change-cd` — sparse diff construction.

use super::ChangeCD;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeCD, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_c_d.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Cd must be a finite number.", Vec::<String>::new());
    }
    if base.c_d == payload.new_c_d {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Cd already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { c_d: Some(payload.new_c_d.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
