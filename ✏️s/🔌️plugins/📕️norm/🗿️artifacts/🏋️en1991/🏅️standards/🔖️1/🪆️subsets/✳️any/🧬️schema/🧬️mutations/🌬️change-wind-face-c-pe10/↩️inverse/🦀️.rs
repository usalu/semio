//! ↩️ `change-wind-face-c-pe10` inverse: restores the previous `c_pe10`.

use crate::change_wind_face_c_pe10::ChangeWindFaceCPe10;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeWindFaceCPe10, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wind_face_c_pe10::ChangeWindFaceCPe10> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeWindFaceCPe10, base: &En1991Snapshot) -> Option<ChangeWindFaceCPe10> {
    let r0 = base.wind_faces.iter().find(|row| row.id == payload.id)?;
    Some(ChangeWindFaceCPe10 { id: payload.id.clone(), new_c_pe10: r0.c_pe10 })
}
