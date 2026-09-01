//! ↩️ `insert-variable-action` — undo is `remove-variable-action` at the (clamped) FINAL-state
//! index the entry landed at, which is also a valid BASE-state index for the follow-up removal.

use super::InsertVariableAction;
use crate::artifacts::en1990::mutations::remove_variable_action;
use crate::artifacts::en1990::{En1990Mutation, En1990Snapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &InsertVariableAction, base: &En1990Snapshot) -> Vec<En1990Mutation> {
    let at = payload.index.min(crate::artifacts::en1990::en1990_qk(base).len());
    vec![En1990Mutation::RemoveVariableAction(remove_variable_action::RemoveVariableAction { index: at })]
}
//#endregion 🔖️Inverse
