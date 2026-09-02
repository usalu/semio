//! ↩️ Inverse for `ChangeLanguage` — reads the BASE language, never the diff.
use super::ChangeLanguage;
use crate::artifacts::writer::schema::mutations::WriterMutation;
use crate::artifacts::writer::WriterSnapshot;

//#region 🔖️Inverse
/// ↩️ Undo restores `base.language_id`.
pub fn inverse(_payload: &ChangeLanguage, base: &WriterSnapshot) -> Vec<WriterMutation> {
    vec![WriterMutation::ChangeLanguage(ChangeLanguage { new_language_id: base.language_id.clone() })]
}
//#endregion 🔖️Inverse
