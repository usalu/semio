//! 🔺️ `change-cs` — sparse diff construction.

use super::mutation::ChangeCS;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ChangeCS, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_c_s.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Cs must be a finite number.", Vec::<String>::new());
    }
    if base.c_s == payload.new_c_s {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Cs already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { c_s: Some(payload.new_c_s.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
