//! ↩️ Inverse for `insert-roofs`.
use crate::insert_roofs::InsertRoofs;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(payload: &InsertRoofs, _base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_roofs::RemoveRoofs> {
    Ok((|| {
    vec![Op::from(crate::remove_roofs::RemoveRoofs { index: payload.index })]

    })())
}
