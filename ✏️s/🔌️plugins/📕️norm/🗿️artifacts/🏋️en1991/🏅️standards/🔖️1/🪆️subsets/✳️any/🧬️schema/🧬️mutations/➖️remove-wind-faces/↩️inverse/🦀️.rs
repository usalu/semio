//! Inverse for `remove-wind-faces`.
use crate::remove_wind_faces::RemoveWindFaces;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(payload: &RemoveWindFaces, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_wind_faces::InsertWindFaces> {
    Ok((|| {
    if payload.index >= base.wind_faces.len() { return Vec::new(); }
    let item = base.wind_faces[payload.index].clone();
    vec![Op::from(crate::insert_wind_faces::InsertWindFaces { index: payload.index, item })]

    })())
}
