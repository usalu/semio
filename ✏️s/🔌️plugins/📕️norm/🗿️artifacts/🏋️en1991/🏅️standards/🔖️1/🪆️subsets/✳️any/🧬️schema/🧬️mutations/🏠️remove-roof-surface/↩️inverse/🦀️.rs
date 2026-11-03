//! ↩️ `remove-roof-surface` inverse: inserts the removed row back at its position.

use crate::remove_roof_surface::RemoveRoofSurface;
use crate::insert_roof_surface::InsertRoofSurface;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &RemoveRoofSurface, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_roof_surface::InsertRoofSurface> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveRoofSurface, base: &En1991Snapshot) -> Option<InsertRoofSurface> {
    let (index, row) = base.roof_surfaces.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertRoofSurface { index, item: row.clone() })
}
