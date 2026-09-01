//! ↩️ `change-system-type` inverse — restores the pre-change `system_type` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din16798::mutations::change_system_type::ChangeSystemType;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeSystemType, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeSystemType(ChangeSystemType { new_system_type: base.system_type.clone() })]
}
//#endregion 🔖️Inverse
