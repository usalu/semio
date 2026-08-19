//! ↩️ `change-importance-class` inverse — restores the pre-change `importance_class` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1998::mutations::change_importance_class::mutation::ChangeImportanceClass;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeImportanceClass, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeImportanceClass(ChangeImportanceClass { new_importance_class: base.importance_class.clone() })]
}
//#endregion 🔖️Inverse
