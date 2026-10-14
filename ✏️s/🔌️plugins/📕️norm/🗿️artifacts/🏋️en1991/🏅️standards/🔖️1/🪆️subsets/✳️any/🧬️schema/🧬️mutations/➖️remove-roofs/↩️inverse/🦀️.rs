//! Inverse for `remove-roofs`.
use crate::remove_roofs::RemoveRoofs;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(payload: &RemoveRoofs, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_roofs::InsertRoofs> {
    Ok((|| {
    if payload.index >= base.roofs.len() { return Vec::new(); }
    let item = base.roofs[payload.index].clone();
    vec![Op::from(crate::insert_roofs::InsertRoofs { index: payload.index, item })]

    })())
}
