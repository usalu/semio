//! 🔺️ `change-t-int-c` diff.

use super::ChangeTIntC;
use crate::{Din4108Diff, Din4108Snapshot};

pub fn diff(payload: &ChangeTIntC, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    if !payload.new_t_int_c.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "t_int_c must be a finite number.", Vec::<String>::new());
    }
    if base.t_int_c == payload.new_t_int_c {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "t_int_c already has this value.");
    }
    protocol::MutationOutcome::new(Din4108Diff { t_int_c: Some(payload.new_t_int_c), ..Default::default() })
}
