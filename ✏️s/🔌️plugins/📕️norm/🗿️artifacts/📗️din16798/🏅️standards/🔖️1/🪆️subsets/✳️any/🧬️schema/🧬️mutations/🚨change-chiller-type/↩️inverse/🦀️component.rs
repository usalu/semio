//! ↩️ `change-chiller-type` inverse — restores the pre-change `chiller_type` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din16798::mutations::change_chiller_type::mutation::ChangeChillerType;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeChillerType, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeChillerType(ChangeChillerType { new_chiller_type: base.chiller_type.clone() })]
}
//#endregion 🔖️Inverse
