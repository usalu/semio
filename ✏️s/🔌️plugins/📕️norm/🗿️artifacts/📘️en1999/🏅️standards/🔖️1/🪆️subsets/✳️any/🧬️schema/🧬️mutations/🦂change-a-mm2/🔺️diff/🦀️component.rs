//! 🔺️ `change-a-mm2` sparse diff construction — writes only `En1999Diff.a_mm2` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_a_mm2::mutation::ChangeAMm2;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAMm2, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_a_mm2.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Cross-section area [mm2] must be a finite number, got {}.", payload.new_a_mm2), Vec::<String>::new());
    }
    if base.a_mm2 == payload.new_a_mm2 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Cross-section area [mm2] is already {}.", payload.new_a_mm2));
    }
    protocol::MutationOutcome::new(En1999Diff { a_mm2: Some(payload.new_a_mm2.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
