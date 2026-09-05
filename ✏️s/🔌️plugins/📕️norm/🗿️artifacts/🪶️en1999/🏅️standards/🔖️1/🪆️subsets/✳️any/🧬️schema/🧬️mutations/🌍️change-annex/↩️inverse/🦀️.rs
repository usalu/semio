//! ↩️ `change-annex` inverse — restores the pre-change `annex` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1999::mutations::change_annex::ChangeAnnex;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeAnnex, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::ChangeAnnex(ChangeAnnex { new_annex: base.annex.clone() })]
}
//#endregion 🔖️Inverse
