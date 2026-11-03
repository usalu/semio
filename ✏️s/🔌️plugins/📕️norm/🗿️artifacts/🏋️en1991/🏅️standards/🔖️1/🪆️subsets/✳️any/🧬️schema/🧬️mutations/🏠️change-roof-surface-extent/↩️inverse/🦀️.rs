//! ↩️ `change-roof-surface-extent` inverse: restores the previous `extent`.

use crate::change_roof_surface_extent::ChangeRoofSurfaceExtent;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeRoofSurfaceExtent, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_roof_surface_extent::ChangeRoofSurfaceExtent> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeRoofSurfaceExtent, base: &En1991Snapshot) -> Option<ChangeRoofSurfaceExtent> {
    let r0 = base.roof_surfaces.iter().find(|row| row.id == payload.id)?;
    Some(ChangeRoofSurfaceExtent { id: payload.id.clone(), new_extent: r0.extent })
}
