//! 🔺️ `change-span-m` sparse diff construction — writes only `En1992Diff.span_m` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_span_m::ChangeSpanM;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeSpanM, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_span_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Span m must be a finite number.", Vec::<String>::new());
    }
    if base.span_m == payload.new_span_m {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Span m already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { span_m: Some(payload.new_span_m.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
