//! ↩️ `change-use-class` inverse — restores the pre-change `use_class` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din18599::mutations::change_use_class::mutation::ChangeUseClass;
use crate::artifacts::din18599::mutations::Din18599Mutation;
use crate::artifacts::din18599::Din18599Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeUseClass, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
    vec![Din18599Mutation::ChangeUseClass(ChangeUseClass { new_use_class: base.use_class.clone() })]
}
//#endregion 🔖️Inverse
