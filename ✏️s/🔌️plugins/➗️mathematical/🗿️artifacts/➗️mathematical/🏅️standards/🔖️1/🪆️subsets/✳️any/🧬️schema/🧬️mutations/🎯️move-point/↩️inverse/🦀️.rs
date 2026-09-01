//! ↩️ `move-point` — undo reconstructed from BASE state; out-of-range index ⇒ `Vec::new()`.

use crate::artifacts::mathematical::{mathematical_geometry, MathematicalMutation, MathematicalSnapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &super::MovePoint, base: &MathematicalSnapshot) -> Vec<MathematicalMutation> {
    let geometry = crate::artifacts::mathematical::mathematical_geometry(base);
    match geometry.points.get(payload.index) {
        Some(point) => vec![MathematicalMutation::MovePoint(super::MovePoint { index: payload.index, x: point.x, y: point.y })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
