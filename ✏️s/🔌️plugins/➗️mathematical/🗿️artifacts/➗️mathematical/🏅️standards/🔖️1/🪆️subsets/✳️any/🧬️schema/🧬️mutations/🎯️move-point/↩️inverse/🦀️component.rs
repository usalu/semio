//! ↩️ `move-point` — undo reconstructed from BASE state; out-of-range index ⇒ `Vec::new()`.

use super::mutation::MovePoint;
use crate::artifacts::mathematical::{MathematicalMutation, MathematicalSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &MovePoint, base: &MathematicalSnapshot) -> Vec<MathematicalMutation> {
    let geometry = crate::artifacts::mathematical::mathematical_geometry(base);
    match geometry.points.get(payload.index) {
        Some(point) => vec![MathematicalMutation::MovePoint(MovePoint { index: payload.index, x: point.x, y: point.y })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
