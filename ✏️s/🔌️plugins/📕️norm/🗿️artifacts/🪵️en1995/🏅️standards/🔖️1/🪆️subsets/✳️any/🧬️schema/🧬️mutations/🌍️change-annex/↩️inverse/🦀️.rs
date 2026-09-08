//! ↩️ `change-annex` inverse — restores the pre-change `annex` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::mutations::set_snapshot::ChangeAnnex;
use crate::mutations::En1995Mutation;
use crate::En1995Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeAnnex, base: &En1995Snapshot) -> Vec<En1995Mutation> {
    vec![En1995Mutation::ChangeAnnex(ChangeAnnex { new_annex: base.annex })]
}
//#endregion 🔖️Inverse
