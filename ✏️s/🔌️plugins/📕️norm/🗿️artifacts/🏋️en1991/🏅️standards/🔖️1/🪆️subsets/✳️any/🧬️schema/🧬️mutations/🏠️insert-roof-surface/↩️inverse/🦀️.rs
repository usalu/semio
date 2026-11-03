//! ↩️ `insert-roof-surface` inverse: removes the inserted row again.

use crate::insert_roof_surface::InsertRoofSurface;
use crate::remove_roof_surface::RemoveRoofSurface;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &InsertRoofSurface, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_roof_surface::RemoveRoofSurface> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertRoofSurface, base: &En1991Snapshot) -> Option<RemoveRoofSurface> {
    Some(RemoveRoofSurface { id: payload.item.id.clone() })
}
