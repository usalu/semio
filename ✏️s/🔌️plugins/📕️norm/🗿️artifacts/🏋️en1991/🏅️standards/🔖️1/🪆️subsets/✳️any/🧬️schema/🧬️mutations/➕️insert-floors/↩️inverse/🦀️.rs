//! Inverse for `insert-floors`.
use crate::insert_floors::InsertFloors;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(payload: &InsertFloors, _base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_floors::RemoveFloors> {
    Ok((|| {
    vec![Op::from(crate::remove_floors::RemoveFloors { index: payload.index })]

    })())
}
