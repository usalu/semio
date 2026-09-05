//! 🔺️ `change-t-int-c` — sparse diff construction.

use super::ChangeTIntC;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeTIntC, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    if !payload.new_t_int_c.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "T int c must be a finite number.", Vec::<String>::new());
    }
    if base.t_int_c == payload.new_t_int_c {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "T int c already has this value.");
    }
    protocol::MutationOutcome::new(Din4108Diff { t_int_c: Some(payload.new_t_int_c.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
