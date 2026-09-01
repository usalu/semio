//! ↩️ `change-l-aeq-db` inverse — restores the pre-change `l_aeq_db` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din16798::mutations::change_l_aeq_db::ChangeLAeqDb;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeLAeqDb, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeLAeqDb(ChangeLAeqDb { new_l_aeq_db: base.l_aeq_db.clone() })]
}
//#endregion 🔖️Inverse
