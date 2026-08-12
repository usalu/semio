//! ↩️ `change-selection-class` — undo restores BASE's class id.

use super::mutation::ChangeSelectionClass;
use crate::artifacts::iso16757::{Iso16757Mutation, Iso16757Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeSelectionClass, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
    vec![Iso16757Mutation::ChangeSelectionClass(ChangeSelectionClass { new_class_id: base.selection.class_id.clone() })]
}
//#endregion 🔖️Inverse
