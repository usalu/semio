//! ↩️ `change-consequence-class` — undo restores BASE's `consequence_class`; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use super::mutation::ChangeConsequenceClass;
use crate::artifacts::en1990::{En1990Mutation, En1990Snapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeConsequenceClass, base: &En1990Snapshot) -> Vec<En1990Mutation> {
    vec![En1990Mutation::ChangeConsequenceClass(ChangeConsequenceClass { new_consequence_class: base.consequence_class })]
}
//#endregion 🔖️Inverse
