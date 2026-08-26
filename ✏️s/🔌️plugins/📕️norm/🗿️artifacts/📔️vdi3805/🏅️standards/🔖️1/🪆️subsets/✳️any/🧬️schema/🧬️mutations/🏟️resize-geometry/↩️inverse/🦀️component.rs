//! ↩️ `resize-geometry` — undo restores BASE's bbox; missing id ⇒ `Vec::new()`.

use super::mutation::ResizeGeometry;
use crate::artifacts::vdi3805::{Vdi3805Mutation, Vdi3805Snapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &ResizeGeometry, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
    let Some(geometry) = base.geometry.get(&payload.id) else {
        return Vec::new();
    };
    vec![Vdi3805Mutation::ResizeGeometry(ResizeGeometry { id: payload.id.clone(), new_bbox: geometry.bbox })]
}
//#endregion 🔖️Inverse
