//! ↩️ `change-wind-face-z` inverse: restores the previous `z`.

use crate::change_wind_face_z::ChangeWindFaceZ;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeWindFaceZ, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wind_face_z::ChangeWindFaceZ> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeWindFaceZ, base: &En1991Snapshot) -> Option<ChangeWindFaceZ> {
    let r0 = base.wind_faces.iter().find(|row| row.id == payload.id)?;
    Some(ChangeWindFaceZ { id: payload.id.clone(), new_z: r0.z })
}
