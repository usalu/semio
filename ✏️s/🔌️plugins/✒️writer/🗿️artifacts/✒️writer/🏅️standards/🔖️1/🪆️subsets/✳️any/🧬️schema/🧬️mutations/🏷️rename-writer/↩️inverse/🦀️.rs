//! ↩️ Inverse for `RenameWriter` — reads the BASE id, never the diff.
use super::RenameWriter;
use crate::WriterSnapshot;

//#region 🔖️Inverse
/// ↩️ Undo restores `base.id`; a document's identity field always has a prior value, so this
/// always yields exactly one restoring mutation.
pub fn inverse<Op>(_payload: &RenameWriter, base: &WriterSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::rename_writer::RenameWriter>,
{
    Ok((|| {
    vec![Op::from(RenameWriter { new_id: base.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
