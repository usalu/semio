//! ↩️ Inverse for `ChangeUri` — reads the BASE uri, never the diff.
use super::ChangeUri;
use crate::schema::mutations::WriterMutation;
use crate::WriterSnapshot;

//#region 🔖️Inverse
/// ↩️ Undo restores `base.uri`.
pub fn inverse(_payload: &ChangeUri, base: &WriterSnapshot) -> Result<Vec<WriterMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![WriterMutation::ChangeUri(ChangeUri { new_uri: base.uri.clone() })]

    })())
}
//#endregion 🔖️Inverse
