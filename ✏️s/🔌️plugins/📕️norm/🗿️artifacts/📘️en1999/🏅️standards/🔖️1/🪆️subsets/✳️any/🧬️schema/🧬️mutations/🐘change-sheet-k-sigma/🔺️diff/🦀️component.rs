//! 🔺️ `change-sheet-k-sigma` sparse diff construction — writes only `En1999Diff.sheet_k_sigma` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_sheet_k_sigma::mutation::ChangeSheetKSigma;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeSheetKSigma, _base: &En1999Snapshot) -> En1999Diff {
    En1999Diff { sheet_k_sigma: Some(payload.new_sheet_k_sigma.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
