//! ↩️ `change-variable-action-category` — undo restores BASE's `category` at that index;
//! out-of-range BASE index ⇒ `Vec::new()`.

use super::mutation::ChangeVariableActionCategory;
use crate::artifacts::en1990::{En1990Mutation, En1990Snapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &ChangeVariableActionCategory, base: &En1990Snapshot) -> Vec<En1990Mutation> {
    match base.q_k.get(payload.index) {
        Some(entry) => vec![En1990Mutation::ChangeVariableActionCategory(ChangeVariableActionCategory { index: payload.index, new_category: entry.category.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
