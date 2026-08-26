//! ↩️ `change-variable-action-value` — undo restores BASE's `value` at that index; out-of-range
//! BASE index ⇒ `Vec::new()`.

use super::mutation::ChangeVariableActionValue;
use crate::artifacts::en1990::{En1990Mutation, En1990Snapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &ChangeVariableActionValue, base: &En1990Snapshot) -> Vec<En1990Mutation> {
    match crate::artifacts::en1990::en1990_qk(base).get(payload.index) {
        Some(entry) => vec![En1990Mutation::ChangeVariableActionValue(ChangeVariableActionValue { index: payload.index, new_value: entry.value })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
