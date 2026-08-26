//! 🔺️ `change-a-mm2` sparse diff construction — writes only `En1995Diff.a_mm2` from the payload.

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::change_a_mm2::mutation::ChangeAMm2;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAMm2, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if !payload.new_a_mm2.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A mm2 must be a finite number.", Vec::<String>::new());
    }
    if base.a_mm2 == payload.new_a_mm2 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "A mm2 already has this value.");
    }
    protocol::MutationOutcome::new(En1995Diff { a_mm2: Some(payload.new_a_mm2.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
