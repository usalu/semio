//! ↩️ `change-chiller-type` inverse — restores the pre-change `chiller_type` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_chiller_type::ChangeChillerType;
use crate::mutations::Din16798Mutation;
use crate::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeChillerType, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeChillerType(ChangeChillerType { new_chiller_type: base.chiller_type.clone() })]
}
//#endregion 🔖️Inverse
