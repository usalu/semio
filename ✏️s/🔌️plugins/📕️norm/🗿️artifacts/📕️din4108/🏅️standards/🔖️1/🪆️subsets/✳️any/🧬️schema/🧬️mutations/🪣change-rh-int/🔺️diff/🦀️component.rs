//! 🔺️ `change-rh-int` — sparse diff construction.

use super::mutation::ChangeRhInt;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ChangeRhInt, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    if !payload.new_rh_int.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Rh int must be a finite number.", Vec::<String>::new());
    }
    if base.rh_int == payload.new_rh_int {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Rh int already has this value.");
    }
    protocol::MutationOutcome::new(Din4108Diff { rh_int: Some(payload.new_rh_int.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
