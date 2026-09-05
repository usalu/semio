//! ↩️ `move-point` — undo reconstructed from BASE state; out-of-range index ⇒ `Vec::new()`.

use crate::artifacts::equation::{equation_geometry, EquationMutation, EquationSnapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &super::MovePoint, base: &EquationSnapshot) -> Vec<EquationMutation> {
    let geometry = crate::artifacts::equation::equation_geometry(base);
    match geometry.points.get(payload.index) {
        Some(point) => vec![EquationMutation::MovePoint(super::MovePoint { index: payload.index, x: point.x, y: point.y })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
