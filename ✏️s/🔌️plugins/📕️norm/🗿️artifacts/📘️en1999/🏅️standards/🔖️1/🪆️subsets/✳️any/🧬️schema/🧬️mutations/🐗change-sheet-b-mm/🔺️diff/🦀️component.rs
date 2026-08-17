//! 🔺️ `change-sheet-b-mm` sparse diff construction — writes only `En1999Diff.sheet_b_mm` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_sheet_b_mm::mutation::ChangeSheetBMm;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeSheetBMm, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_sheet_b_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Sheet width b [mm] must be a finite number, got {}.", payload.new_sheet_b_mm), Vec::<String>::new());
    }
    if base.sheet_b_mm == payload.new_sheet_b_mm {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Sheet width b [mm] is already {}.", payload.new_sheet_b_mm));
    }
    protocol::MutationOutcome::new(En1999Diff { sheet_b_mm: Some(payload.new_sheet_b_mm.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
