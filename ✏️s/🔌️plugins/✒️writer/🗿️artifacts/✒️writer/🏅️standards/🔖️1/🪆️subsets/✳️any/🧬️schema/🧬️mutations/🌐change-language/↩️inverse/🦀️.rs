//! ↩️ Inverse for `ChangeLanguage` — reads the BASE language, never the diff.
use super::ChangeLanguage;
use crate::schema::mutations::WriterMutation;
use crate::WriterSnapshot;

//#region 🔖️Inverse
/// ↩️ Undo restores `base.language_id`.
pub fn inverse(_payload: &ChangeLanguage, base: &WriterSnapshot) -> Result<Vec<WriterMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![WriterMutation::ChangeLanguage(ChangeLanguage { new_language_id: base.language_id.clone() })]

    })())
}
//#endregion 🔖️Inverse
