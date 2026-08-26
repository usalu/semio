//! ↩️ `remove-part-number-input` — undo restores the BASE value via `change`; missing key ⇒
//! `Vec::new()`.

use super::mutation::RemovePartNumberInput;
use crate::artifacts::iso16757::mutations::change_part_number_input;
use crate::artifacts::iso16757::{Iso16757Mutation, Iso16757Snapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &RemovePartNumberInput, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
    match base.part_number_inputs.get(&payload.key) {
        Some(old_value) => vec![Iso16757Mutation::ChangePartNumberInput(change_part_number_input::mutation::ChangePartNumberInput { key: payload.key.clone(), new_value: old_value.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
