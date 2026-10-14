//! Inverse for `insert-wind-faces`.
use crate::insert_wind_faces::InsertWindFaces;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(payload: &InsertWindFaces, _base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_wind_faces::RemoveWindFaces> {
    Ok((|| {
    vec![Op::from(crate::remove_wind_faces::RemoveWindFaces { index: payload.index })]

    })())
}
