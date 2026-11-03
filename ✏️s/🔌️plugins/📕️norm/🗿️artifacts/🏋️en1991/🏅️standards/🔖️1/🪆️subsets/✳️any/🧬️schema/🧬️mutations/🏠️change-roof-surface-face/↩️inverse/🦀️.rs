//! ↩️ `change-roof-surface-face` inverse: restores the previous `face`.

use crate::change_roof_surface_face::ChangeRoofSurfaceFace;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeRoofSurfaceFace, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_roof_surface_face::ChangeRoofSurfaceFace> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeRoofSurfaceFace, base: &En1991Snapshot) -> Option<ChangeRoofSurfaceFace> {
    let r0 = base.roof_surfaces.iter().find(|row| row.id == payload.id)?;
    Some(ChangeRoofSurfaceFace { id: payload.id.clone(), new_face: r0.face.clone() })
}
