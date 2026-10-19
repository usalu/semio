//! ↩️ Inverse for `change-wind-face-assumed-wp`.
use crate::change_wind_face_assumed_wp::ChangeWindFaceAssumedWp;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(payload: &ChangeWindFaceAssumedWp, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wind_face_assumed_wp::ChangeWindFaceAssumedWp> {
    Ok((|| {
    if payload.index >= base.wind_faces.len() { return Vec::new(); }
    vec![Op::from(ChangeWindFaceAssumedWp { index: payload.index, new_assumed_wp: base.wind_faces[payload.index].assumed_wp })]

    })())
}
