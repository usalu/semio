//! ↩️ `change-wind-face-cs` inverse: restores the previous `c_s`.

use crate::change_wind_face_c_s::ChangeWindFaceCS;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeWindFaceCS, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wind_face_c_s::ChangeWindFaceCS> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeWindFaceCS, base: &En1991Snapshot) -> Option<ChangeWindFaceCS> {
    let r0 = base.wind_faces.iter().find(|row| row.id == payload.id)?;
    Some(ChangeWindFaceCS { id: payload.id.clone(), new_c_s: r0.c_s })
}
