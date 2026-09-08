//! ↩️ `remove-variable-action` — undo re-`insert`s the captured entry at its original BASE-state
//! index; out-of-range BASE index ⇒ `Vec::new()`.

use super::RemoveVariableAction;
use crate::mutations::insert_variable_action;
use crate::{En1990Mutation, En1990Snapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &RemoveVariableAction, base: &En1990Snapshot) -> Vec<En1990Mutation> {
    match crate::en1990_qk(base).get(payload.index) {
        Some(entry) => vec![En1990Mutation::InsertVariableAction(insert_variable_action::InsertVariableAction { index: payload.index, category: entry.category.clone(), value: entry.value })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
