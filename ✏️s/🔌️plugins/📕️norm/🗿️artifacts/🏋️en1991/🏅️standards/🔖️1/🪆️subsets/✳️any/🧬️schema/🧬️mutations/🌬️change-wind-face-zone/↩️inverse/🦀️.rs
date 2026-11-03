//! ↩️ `change-wind-face-zone` inverse: restores the previous `zone`.

use crate::change_wind_face_zone::ChangeWindFaceZone;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeWindFaceZone, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wind_face_zone::ChangeWindFaceZone> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeWindFaceZone, base: &En1991Snapshot) -> Option<ChangeWindFaceZone> {
    let r0 = base.wind_faces.iter().find(|row| row.id == payload.id)?;
    Some(ChangeWindFaceZone { id: payload.id.clone(), new_zone: r0.zone.clone() })
}
