//! ↩️ Inverse for `ChangeLanguage` — reads the BASE language, never the diff.
use super::ChangeLanguage;
use crate::WriterSnapshot;

//#region 🔖️Inverse
/// ↩️ Undo restores `base.language_id`.
pub fn inverse<Op>(_payload: &ChangeLanguage, base: &WriterSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_language::ChangeLanguage>,
{
    Ok((|| {
    vec![Op::from(ChangeLanguage { new_language_id: base.language_id.clone() })]

    })())
}
//#endregion 🔖️Inverse
