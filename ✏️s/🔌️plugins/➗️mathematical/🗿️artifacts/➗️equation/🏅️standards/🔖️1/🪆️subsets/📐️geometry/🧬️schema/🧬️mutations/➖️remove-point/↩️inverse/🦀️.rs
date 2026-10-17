//! ↩️ `remove-point` — undo re-`insert`s the exact point captured from BASE state; out-of-range
//! index ⇒ `Vec::new()`.

use crate::standards::v1::subsets::geometry::schema::mutations::insert_point;
use crate::EquationSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &super::RemovePoint, base: &EquationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::geometry::schema::mutations::insert_point::InsertPoint>,
{
    Ok((|| {
    let geometry = &base.geometry;
    match geometry.points.get(payload.index) {
        Some(point) => vec![Op::from(insert_point::InsertPoint { index: payload.index, x: point.x, y: point.y })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
