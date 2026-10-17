//! ↩️ Inverse for `EditText` — reads the BASE text, never the diff.
use super::EditText;
use crate::WriterSnapshot;

//#region 🔖️Inverse
/// ↩️ Undo restores `base`'s document text wholesale from `base.document`'s local owner
/// (never the diff), failing soft to empty text when a decoded child remains unresolved.
pub fn inverse<Op>(_payload: &EditText, base: &WriterSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::edit_text::EditText>,
{
    Ok((|| {
    vec![Op::from(EditText { text: crate::writer_text(base) })]

    })())
}
//#endregion 🔖️Inverse
