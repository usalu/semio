//! ↩️ `change-annex` inverse — restores the pre-change `annex` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1995::mutations::set_snapshot::mutation::ChangeAnnex;
use crate::artifacts::en1995::mutations::En1995Mutation;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeAnnex, base: &En1995Snapshot) -> Vec<En1995Mutation> {
    vec![En1995Mutation::ChangeAnnex(ChangeAnnex { new_annex: base.annex.clone() })]
}
//#endregion 🔖️Inverse
