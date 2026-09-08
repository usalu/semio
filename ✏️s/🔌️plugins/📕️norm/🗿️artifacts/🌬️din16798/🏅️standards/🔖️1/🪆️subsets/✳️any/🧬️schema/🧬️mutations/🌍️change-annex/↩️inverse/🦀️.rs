//! ↩️ `change-annex` inverse — restores the pre-change `annex` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_annex::ChangeAnnex;
use crate::mutations::Din16798Mutation;
use crate::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeAnnex, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeAnnex(ChangeAnnex { new_annex: base.annex })]
}
//#endregion 🔖️Inverse
