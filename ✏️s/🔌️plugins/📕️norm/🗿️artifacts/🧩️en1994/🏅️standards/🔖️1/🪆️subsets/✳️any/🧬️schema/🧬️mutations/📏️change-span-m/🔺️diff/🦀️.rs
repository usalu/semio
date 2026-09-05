//! 🔺️ `change-span-m` — sparse diff construction.

use super::ChangeSpanM;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeSpanM, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if !payload.new_span_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Span m must be a finite number.", Vec::<String>::new());
    }
    if base.span_m == payload.new_span_m {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Span m already has this value.");
    }
    protocol::MutationOutcome::new(En1994Diff { span_m: Some(payload.new_span_m.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
