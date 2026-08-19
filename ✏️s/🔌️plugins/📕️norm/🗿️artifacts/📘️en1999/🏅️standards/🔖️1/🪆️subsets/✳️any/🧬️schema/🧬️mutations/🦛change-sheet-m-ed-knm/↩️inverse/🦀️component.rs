//! ↩️ `change-sheet-m-ed-knm` inverse — restores the pre-change `sheet_m_ed_knm` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1999::mutations::change_sheet_m_ed_knm::mutation::ChangeSheetMEdKnm;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeSheetMEdKnm, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::ChangeSheetMEdKnm(ChangeSheetMEdKnm { new_sheet_m_ed_knm: base.sheet_m_ed_knm.clone() })]
}
//#endregion 🔖️Inverse
