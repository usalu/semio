//! ↩️ Inverse for `ChangeUri` — reads the BASE uri, never the diff.
use super::mutation::ChangeUri;
use crate::artifacts::writer::schema::mutations::WriterMutation;
use crate::artifacts::writer::WriterSnapshot;

//#region 🔖️Inverse
/// ↩️ Undo restores `base.uri`.
pub async fn inverse(_payload: &ChangeUri, base: &WriterSnapshot) -> Vec<WriterMutation> {
    vec![WriterMutation::ChangeUri(ChangeUri { new_uri: base.uri.clone() })]
}
//#endregion 🔖️Inverse
