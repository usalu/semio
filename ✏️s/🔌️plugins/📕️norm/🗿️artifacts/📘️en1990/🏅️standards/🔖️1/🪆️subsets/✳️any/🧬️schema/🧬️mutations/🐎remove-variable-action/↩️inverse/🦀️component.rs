//! ↩️ `remove-variable-action` — undo re-`insert`s the captured entry at its original BASE-state
//! index; out-of-range BASE index ⇒ `Vec::new()`.

use super::mutation::RemoveVariableAction;
use crate::artifacts::en1990::mutations::insert_variable_action;
use crate::artifacts::en1990::{En1990Mutation, En1990Snapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &RemoveVariableAction, base: &En1990Snapshot) -> Vec<En1990Mutation> {
    match crate::artifacts::en1990::en1990_qk(base).get(payload.index) {
        Some(entry) => vec![En1990Mutation::InsertVariableAction(insert_variable_action::mutation::InsertVariableAction { index: payload.index, category: entry.category.clone(), value: entry.value })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
