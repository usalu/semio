//! ↩️ `remove-part-number-input` — undo restores the BASE value via `change`; missing key ⇒
//! `Vec::new()`.

use crate::remove_part_number_input::mutation::RemovePartNumberInput;
use crate::change_part_number_input;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &RemovePartNumberInput, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_part_number_input::mutation::ChangePartNumberInput> {
    Ok((|| {
    match base.part_number_inputs.get(&payload.key) {
        Some(old_value) => vec![Op::from(change_part_number_input::mutation::ChangePartNumberInput { key: payload.key.clone(), new_value: old_value.clone() })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
