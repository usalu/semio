//! 🔺️ `change-rh-int` diff.

use super::ChangeRhInt;
use crate::{Din4108Diff, Din4108Snapshot};

pub fn diff(payload: &ChangeRhInt, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    if !payload.new_rh_int.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "rh_int must be a finite number.", Vec::<String>::new());
    }
    if base.rh_int == payload.new_rh_int {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "rh_int already has this value.");
    }
    protocol::MutationOutcome::new(Din4108Diff { rh_int: Some(payload.new_rh_int), ..Default::default() })
}
