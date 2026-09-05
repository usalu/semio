//! ↩️ `change-duct-class` inverse — restores the pre-change `duct_class` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din16798::mutations::change_duct_class::ChangeDuctClass;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeDuctClass, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeDuctClass(ChangeDuctClass { new_duct_class: base.duct_class.clone() })]
}
//#endregion 🔖️Inverse
