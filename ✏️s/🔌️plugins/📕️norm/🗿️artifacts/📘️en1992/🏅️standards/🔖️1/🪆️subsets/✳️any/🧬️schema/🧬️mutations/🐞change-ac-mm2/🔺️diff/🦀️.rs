//! 🔺️ `change-ac-mm2` sparse diff construction — writes only `En1992Diff.a_c_mm2` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_a_c_mm2::ChangeACMm2;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeACMm2, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_a_c_mm2.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Ac mm2 must be a finite number.", Vec::<String>::new());
    }
    if base.a_c_mm2 == payload.new_a_c_mm2 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Ac mm2 already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { a_c_mm2: Some(payload.new_a_c_mm2.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
