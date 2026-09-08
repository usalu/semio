//! ↩️ `change-sheet-k-sigma` inverse — restores the pre-change `sheet_k_sigma` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_sheet_k_sigma::ChangeSheetKSigma;
use crate::mutations::En1999Mutation;
use crate::En1999Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeSheetKSigma, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::ChangeSheetKSigma(ChangeSheetKSigma { new_sheet_k_sigma: base.sheet_k_sigma })]
}
//#endregion 🔖️Inverse
