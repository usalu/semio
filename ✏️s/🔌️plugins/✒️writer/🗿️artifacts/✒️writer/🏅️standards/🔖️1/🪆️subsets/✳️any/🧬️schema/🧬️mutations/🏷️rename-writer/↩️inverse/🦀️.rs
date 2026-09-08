//! ↩️ Inverse for `RenameWriter` — reads the BASE id, never the diff.
use super::RenameWriter;
use crate::schema::mutations::WriterMutation;
use crate::WriterSnapshot;

//#region 🔖️Inverse
/// ↩️ Undo restores `base.id`; a document's identity field always has a prior value, so this
/// always yields exactly one restoring mutation.
pub fn inverse(_payload: &RenameWriter, base: &WriterSnapshot) -> Vec<WriterMutation> {
    vec![WriterMutation::RenameWriter(RenameWriter { new_id: base.id.clone() })]
}
//#endregion 🔖️Inverse
