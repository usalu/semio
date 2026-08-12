//! ↩️ Inverse for `EditText` — reads the BASE text, never the diff.
use super::mutation::EditText;
use crate::artifacts::writer::schema::mutations::WriterMutation;
use crate::artifacts::writer::WriterSnapshot;

//#region 🔖️Inverse
/// ↩️ Undo restores `base.text` wholesale.
pub fn inverse(_payload: &EditText, base: &WriterSnapshot) -> Vec<WriterMutation> {
    vec![WriterMutation::EditText(EditText { text: base.text.clone() })]
}
//#endregion 🔖️Inverse
