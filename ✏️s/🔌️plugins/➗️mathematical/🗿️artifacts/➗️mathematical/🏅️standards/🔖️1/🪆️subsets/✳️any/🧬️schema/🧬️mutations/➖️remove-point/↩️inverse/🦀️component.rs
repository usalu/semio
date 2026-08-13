//! ↩️ `remove-point` — undo re-`insert`s the exact point captured from BASE state; out-of-range
//! index ⇒ `Vec::new()`.

use crate::artifacts::mathematical::mutations::insert_point;
use crate::artifacts::mathematical::{MathematicalMutation, MathematicalSnapshot};

use super::mutation::RemovePoint;

//#region 🔖️Inverse
pub fn inverse(payload: &RemovePoint, base: &MathematicalSnapshot) -> Vec<MathematicalMutation> {
    let geometry = crate::artifacts::mathematical::mathematical_geometry(base);
    match geometry.points.get(payload.index) {
        Some(point) => vec![MathematicalMutation::InsertPoint(insert_point::mutation::InsertPoint { index: payload.index, x: point.x, y: point.y })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
