//! 🔺️ `change-sheet-t-mm` sparse diff construction — writes only `En1999Diff.sheet_t_mm` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_sheet_t_mm::mutation::ChangeSheetTMm;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeSheetTMm, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_sheet_t_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Sheet thickness t [mm] must be a finite number, got {}.", payload.new_sheet_t_mm), Vec::<String>::new());
    }
    if base.sheet_t_mm == payload.new_sheet_t_mm {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Sheet thickness t [mm] is already {}.", payload.new_sheet_t_mm));
    }
    protocol::MutationOutcome::new(En1999Diff { sheet_t_mm: Some(payload.new_sheet_t_mm.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
