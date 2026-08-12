//! ↩️ `insert-variable-action` — undo is `remove-variable-action` at the (clamped) FINAL-state
//! index the entry landed at, which is also a valid BASE-state index for the follow-up removal.

use super::mutation::InsertVariableAction;
use crate::artifacts::en1990::mutations::remove_variable_action;
use crate::artifacts::en1990::{En1990Mutation, En1990Snapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &InsertVariableAction, base: &En1990Snapshot) -> Vec<En1990Mutation> {
    let at = payload.index.min(base.q_k.len());
    vec![En1990Mutation::RemoveVariableAction(remove_variable_action::mutation::RemoveVariableAction { index: at })]
}
//#endregion 🔖️Inverse
