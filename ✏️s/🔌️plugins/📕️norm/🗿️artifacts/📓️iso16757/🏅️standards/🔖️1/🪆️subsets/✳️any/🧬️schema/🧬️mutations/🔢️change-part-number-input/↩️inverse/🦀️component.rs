//! ↩️ `change-part-number-input` — undo restores the BASE value, or `remove`s the key if it was
//! previously absent (this mutation upserts, so a fresh key's undo is `remove`, not `change`).

use super::mutation::ChangePartNumberInput;
use crate::artifacts::iso16757::mutations::remove_part_number_input;
use crate::artifacts::iso16757::{Iso16757Mutation, Iso16757Snapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &ChangePartNumberInput, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
    match base.part_number_inputs.get(&payload.key) {
        Some(old_value) => vec![Iso16757Mutation::ChangePartNumberInput(ChangePartNumberInput { key: payload.key.clone(), new_value: old_value.clone() })],
        None => vec![Iso16757Mutation::RemovePartNumberInput(remove_part_number_input::mutation::RemovePartNumberInput { key: payload.key.clone() })],
    }
}
//#endregion 🔖️Inverse
