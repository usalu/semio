//! ↩️ `remove-geometry` — undo re-`create`s the geometry from BASE state; missing id ⇒
//! `Vec::new()`.

use crate::remove_geometry::RemoveGeometry;
use crate::add_geometry;
use semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &RemoveGeometry, base: &Vdi3805Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::add_geometry::AddGeometry> {
    Ok((|| {
    match base.geometry.get(&payload.id) {
        Some(geometry) => vec![Op::from(add_geometry::AddGeometry { geometry: geometry.clone() })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
