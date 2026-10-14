//! ↩️ `resize-geometry` — undo restores BASE's bbox; missing id ⇒ `Vec::new()`.

use crate::resize_geometry::ResizeGeometry;
use semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &ResizeGeometry, base: &Vdi3805Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::resize_geometry::ResizeGeometry> {
    Ok((|| {
    let Some(geometry) = base.geometry.get(&payload.id) else {
        return Vec::new();
    };
    vec![Op::from(ResizeGeometry { id: payload.id.clone(), new_bbox: geometry.bbox })]

    })())
}
//#endregion 🔖️Inverse
