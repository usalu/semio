//! ↩️ `change-wind-face-cd` inverse: restores the previous `c_d`.

use crate::change_wind_face_c_d::ChangeWindFaceCD;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeWindFaceCD, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wind_face_c_d::ChangeWindFaceCD> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeWindFaceCD, base: &En1991Snapshot) -> Option<ChangeWindFaceCD> {
    let r0 = base.wind_faces.iter().find(|row| row.id == payload.id)?;
    Some(ChangeWindFaceCD { id: payload.id.clone(), new_c_d: r0.c_d })
}
