//! ↩️ `add-geometry` — undo is `remove-geometry`, unless `base` already had this id (then
//! `create` was a no-op).

use crate::add_geometry::AddGeometry;
use crate::remove_geometry;
use semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &AddGeometry, base: &Vdi3805Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_geometry::RemoveGeometry> {
    Ok((|| {
    if base.geometry.contains_key(&payload.geometry.id) {
        return Vec::new();
    }
    vec![Op::from(remove_geometry::RemoveGeometry { id: payload.geometry.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
