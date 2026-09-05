//! 🔺️ `change-weld-length-mm` sparse diff construction — writes only `En1999Diff.weld_length_mm` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_weld_length_mm::ChangeWeldLengthMm;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeWeldLengthMm, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_weld_length_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Weld length [mm] must be a finite number, got {}.", payload.new_weld_length_mm), Vec::<String>::new());
    }
    if base.weld_length_mm == payload.new_weld_length_mm {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Weld length [mm] is already {}.", payload.new_weld_length_mm));
    }
    protocol::MutationOutcome::new(En1999Diff { weld_length_mm: Some(payload.new_weld_length_mm.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
