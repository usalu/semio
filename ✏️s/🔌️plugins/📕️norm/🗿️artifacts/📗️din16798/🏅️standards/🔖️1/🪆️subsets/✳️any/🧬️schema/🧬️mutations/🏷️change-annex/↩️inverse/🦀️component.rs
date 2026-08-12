//! ↩️ `change-annex` inverse — restores the pre-change `annex` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din16798::mutations::change_annex::mutation::ChangeAnnex;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeAnnex, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeAnnex(ChangeAnnex { new_annex: base.annex.clone() })]
}
//#endregion 🔖️Inverse
