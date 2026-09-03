//! ↩️ `remove-point` — undo re-`insert`s the exact point captured from BASE state; out-of-range
//! index ⇒ `Vec::new()`.

use crate::artifacts::equation::standards::v1::subsets::geometry::schema::mutations::insert_point;
use crate::artifacts::equation::{equation_geometry, EquationMutation, EquationSnapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &super::RemovePoint, base: &EquationSnapshot) -> Vec<EquationMutation> {
    let geometry = crate::artifacts::equation::equation_geometry(base);
    match geometry.points.get(payload.index) {
        Some(point) => vec![EquationMutation::InsertPoint(insert_point::InsertPoint { index: payload.index, x: point.x, y: point.y })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
