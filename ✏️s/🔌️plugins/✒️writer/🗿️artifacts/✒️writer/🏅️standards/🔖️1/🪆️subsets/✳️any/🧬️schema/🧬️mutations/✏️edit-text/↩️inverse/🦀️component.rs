//! ↩️ Inverse for `EditText` — reads the BASE text, never the diff.
use super::mutation::EditText;
use crate::artifacts::writer::schema::mutations::WriterMutation;
use crate::artifacts::writer::WriterSnapshot;

//#region 🔖️Inverse
/// ↩️ Undo restores `base`'s document text wholesale from `base.document`'s local owner
/// (never the diff), failing soft to empty text when a decoded child remains unresolved.
pub fn inverse(_payload: &EditText, base: &WriterSnapshot) -> Vec<WriterMutation> {
    vec![WriterMutation::EditText(EditText { text: crate::artifacts::writer::writer_text(base) })]
}
//#endregion 🔖️Inverse
