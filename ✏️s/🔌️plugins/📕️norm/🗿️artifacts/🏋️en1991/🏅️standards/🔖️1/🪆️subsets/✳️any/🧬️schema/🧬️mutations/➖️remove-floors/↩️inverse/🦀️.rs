//! ↩️ Inverse for `remove-floors`.
use crate::remove_floors::RemoveFloors;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(payload: &RemoveFloors, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_floors::InsertFloors> {
    Ok((|| {
    if payload.index >= base.floors.len() { return Vec::new(); }
    let item = base.floors[payload.index].clone();
    vec![Op::from(crate::insert_floors::InsertFloors { index: payload.index, item })]

    })())
}
