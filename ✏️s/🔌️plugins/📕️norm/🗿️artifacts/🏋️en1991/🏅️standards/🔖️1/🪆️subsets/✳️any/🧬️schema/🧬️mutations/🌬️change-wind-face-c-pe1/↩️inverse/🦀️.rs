//! ↩️ `change-wind-face-c-pe1` inverse: restores the previous `c_pe1`.

use crate::change_wind_face_c_pe1::ChangeWindFaceCPe1;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeWindFaceCPe1, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wind_face_c_pe1::ChangeWindFaceCPe1> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeWindFaceCPe1, base: &En1991Snapshot) -> Option<ChangeWindFaceCPe1> {
    let r0 = base.wind_faces.iter().find(|row| row.id == payload.id)?;
    Some(ChangeWindFaceCPe1 { id: payload.id.clone(), new_c_pe1: r0.c_pe1 })
}
