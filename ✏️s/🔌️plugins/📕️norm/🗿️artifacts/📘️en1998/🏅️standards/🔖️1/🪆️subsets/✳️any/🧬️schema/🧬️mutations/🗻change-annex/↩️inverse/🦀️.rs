//! ↩️ `change-annex` inverse — restores the pre-change `annex` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1998::mutations::change_annex::ChangeAnnex;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeAnnex, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeAnnex(ChangeAnnex { new_annex: base.annex.clone() })]
}
//#endregion 🔖️Inverse
