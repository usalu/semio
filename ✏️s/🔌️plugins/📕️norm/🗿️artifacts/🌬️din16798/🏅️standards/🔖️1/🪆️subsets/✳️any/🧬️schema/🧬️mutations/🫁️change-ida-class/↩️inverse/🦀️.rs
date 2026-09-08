//! ↩️ `change-ida-class` inverse — restores the pre-change `ida_class` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_ida_class::ChangeIdaClass;
use crate::mutations::Din16798Mutation;
use crate::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeIdaClass, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeIdaClass(ChangeIdaClass { new_ida_class: base.ida_class.clone() })]
}
//#endregion 🔖️Inverse
