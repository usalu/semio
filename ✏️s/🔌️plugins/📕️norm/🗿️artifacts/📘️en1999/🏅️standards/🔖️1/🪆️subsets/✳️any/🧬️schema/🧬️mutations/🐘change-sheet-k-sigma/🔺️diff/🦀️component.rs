//! 🔺️ `change-sheet-k-sigma` sparse diff construction — writes only `En1999Diff.sheet_k_sigma` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_sheet_k_sigma::mutation::ChangeSheetKSigma;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeSheetKSigma, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_sheet_k_sigma.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Sheet plate buckling factor k_sigma must be a finite number, got {}.", payload.new_sheet_k_sigma), Vec::<String>::new());
    }
    if base.sheet_k_sigma == payload.new_sheet_k_sigma {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Sheet plate buckling factor k_sigma is already {}.", payload.new_sheet_k_sigma));
    }
    protocol::MutationOutcome::new(En1999Diff { sheet_k_sigma: Some(payload.new_sheet_k_sigma.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
