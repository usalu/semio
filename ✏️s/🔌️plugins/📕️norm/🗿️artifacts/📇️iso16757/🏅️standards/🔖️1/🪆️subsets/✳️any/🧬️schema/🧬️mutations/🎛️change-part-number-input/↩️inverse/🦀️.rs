//! ↩️ `change-part-number-input` — undo restores the BASE value, or `remove`s the key if it was
//! previously absent (this mutation upserts, so a fresh key's undo is `remove`, not `change`).

use crate::change_part_number_input::mutation::ChangePartNumberInput;
use crate::remove_part_number_input;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &ChangePartNumberInput, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_part_number_input::mutation::ChangePartNumberInput> + From<crate::remove_part_number_input::mutation::RemovePartNumberInput> {
    Ok((|| {
    match base.part_number_inputs.get(&payload.key) {
        Some(old_value) => vec![Op::from(ChangePartNumberInput { key: payload.key.clone(), new_value: old_value.clone() })],
        None => vec![Op::from(remove_part_number_input::mutation::RemovePartNumberInput { key: payload.key.clone() })],
    }

    })())
}
//#endregion 🔖️Inverse
