//! ↩️ Inverse for `EditText` — reads the BASE text, never the diff.
use super::mutation::EditText;
use crate::artifacts::writer::schema::mutations::WriterMutation;
use crate::artifacts::writer::WriterSnapshot;

//#region 🔖️Inverse
/// ↩️ Undo restores `base`'s document text wholesale, read from the working-scene cache off
/// `base.document` (never the diff) — same documented staleness caveat as lowpoly's
/// `StaleMeshWorkspace` gap: fails soft to an empty string if nothing cached this handle's content.
pub fn inverse(_payload: &EditText, base: &WriterSnapshot) -> Vec<WriterMutation> {
    vec![WriterMutation::EditText(EditText { text: crate::artifacts::writer::writer_text(base) })]
}
//#endregion 🔖️Inverse
