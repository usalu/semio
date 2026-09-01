//! ↩️ `remove-point` — undo re-`insert`s the exact point captured from BASE state; out-of-range
//! index ⇒ `Vec::new()`.

use crate::artifacts::mathematical::mutations::insert_point;
use crate::artifacts::mathematical::{mathematical_geometry, MathematicalMutation, MathematicalSnapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &super::RemovePoint, base: &MathematicalSnapshot) -> Vec<MathematicalMutation> {
    let geometry = crate::artifacts::mathematical::mathematical_geometry(base);
    match geometry.points.get(payload.index) {
        Some(point) => vec![MathematicalMutation::InsertPoint(insert_point::InsertPoint { index: payload.index, x: point.x, y: point.y })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
