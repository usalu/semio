//! ↩️ `remove-point` — undo re-`insert`s the exact point captured from BASE state; out-of-range
//! index ⇒ `Vec::new()`.

use crate::standards::v1::subsets::geometry::schema::mutations::insert_point;
use crate::{EquationMutation, EquationSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &super::RemovePoint, base: &EquationSnapshot) -> Result<Vec<EquationMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let geometry = base.geometry.clone();
    match geometry.points.get(payload.index) {
        Some(point) => vec![EquationMutation::InsertPoint(insert_point::InsertPoint { index: payload.index, x: point.x, y: point.y })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
