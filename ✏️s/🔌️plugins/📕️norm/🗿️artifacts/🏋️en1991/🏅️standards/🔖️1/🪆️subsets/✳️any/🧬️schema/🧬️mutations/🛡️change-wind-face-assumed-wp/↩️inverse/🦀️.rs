//! ↩️ `change-wind-face-assumed-wp` inverse: restores the previous `assumed_wp`.

use crate::change_wind_face_assumed_wp::ChangeWindFaceAssumedWp;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeWindFaceAssumedWp, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wind_face_assumed_wp::ChangeWindFaceAssumedWp> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeWindFaceAssumedWp, base: &En1991Snapshot) -> Option<ChangeWindFaceAssumedWp> {
    let r0 = base.wind_faces.iter().find(|row| row.id == payload.id)?;
    Some(ChangeWindFaceAssumedWp { id: payload.id.clone(), new_assumed_wp: r0.assumed_wp })
}
