//! ↩️ `change-roof-surface-shape` inverse: restores the previous `shape`.

use crate::change_roof_surface_shape::ChangeRoofSurfaceShape;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeRoofSurfaceShape, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_roof_surface_shape::ChangeRoofSurfaceShape> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeRoofSurfaceShape, base: &En1991Snapshot) -> Option<ChangeRoofSurfaceShape> {
    let r0 = base.roof_surfaces.iter().find(|row| row.id == payload.id)?;
    Some(ChangeRoofSurfaceShape { id: payload.id.clone(), new_shape: r0.shape.clone() })
}
