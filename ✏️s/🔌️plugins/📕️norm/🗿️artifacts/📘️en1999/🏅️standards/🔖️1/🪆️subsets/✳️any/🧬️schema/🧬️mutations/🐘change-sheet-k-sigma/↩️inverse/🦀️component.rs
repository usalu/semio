//! ↩️ `change-sheet-k-sigma` inverse — restores the pre-change `sheet_k_sigma` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1999::mutations::change_sheet_k_sigma::mutation::ChangeSheetKSigma;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeSheetKSigma, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::ChangeSheetKSigma(ChangeSheetKSigma { new_sheet_k_sigma: base.sheet_k_sigma.clone() })]
}
//#endregion 🔖️Inverse
