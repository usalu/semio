//! ↩️ `change-annex` inverse — restores the pre-change `annex` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1990::mutations::set_snapshot::mutation::ChangeAnnex;
use crate::artifacts::en1990::mutations::En1990Mutation;
use crate::artifacts::en1990::En1990Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeAnnex, base: &En1990Snapshot) -> Vec<En1990Mutation> {
    vec![En1990Mutation::ChangeAnnex(ChangeAnnex { new_annex: base.annex })]
}
//#endregion 🔖️Inverse
