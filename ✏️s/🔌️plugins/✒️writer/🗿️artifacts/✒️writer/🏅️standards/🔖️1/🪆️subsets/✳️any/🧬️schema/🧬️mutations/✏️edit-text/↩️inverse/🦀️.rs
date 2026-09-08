//! ↩️ Inverse for `EditText` — reads the BASE text, never the diff.
use super::EditText;
use crate::schema::mutations::WriterMutation;
use crate::WriterSnapshot;

//#region 🔖️Inverse
/// ↩️ Undo restores `base`'s document text wholesale from `base.document`'s local owner
/// (never the diff), failing soft to empty text when a decoded child remains unresolved.
pub fn inverse(_payload: &EditText, base: &WriterSnapshot) -> Vec<WriterMutation> {
    vec![WriterMutation::EditText(EditText { text: crate::writer_text(base) })]
}
//#endregion 🔖️Inverse
