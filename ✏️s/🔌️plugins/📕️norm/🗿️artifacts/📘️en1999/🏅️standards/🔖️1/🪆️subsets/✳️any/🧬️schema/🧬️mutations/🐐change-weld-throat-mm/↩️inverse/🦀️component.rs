//! ↩️ `change-weld-throat-mm` inverse — restores the pre-change `weld_throat_mm` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1999::mutations::change_weld_throat_mm::mutation::ChangeWeldThroatMm;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeWeldThroatMm, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::ChangeWeldThroatMm(ChangeWeldThroatMm { new_weld_throat_mm: base.weld_throat_mm.clone() })]
}
//#endregion 🔖️Inverse
