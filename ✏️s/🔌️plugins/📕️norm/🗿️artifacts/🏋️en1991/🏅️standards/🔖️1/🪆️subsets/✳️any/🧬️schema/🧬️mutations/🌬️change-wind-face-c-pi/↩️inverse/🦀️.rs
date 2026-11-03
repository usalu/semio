//! ↩️ `change-wind-face-c-pi` inverse: restores the previous `c_pi`.

use crate::change_wind_face_c_pi::ChangeWindFaceCPi;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeWindFaceCPi, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wind_face_c_pi::ChangeWindFaceCPi> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeWindFaceCPi, base: &En1991Snapshot) -> Option<ChangeWindFaceCPi> {
    let r0 = base.wind_faces.iter().find(|row| row.id == payload.id)?;
    Some(ChangeWindFaceCPi { id: payload.id.clone(), new_c_pi: r0.c_pi })
}
