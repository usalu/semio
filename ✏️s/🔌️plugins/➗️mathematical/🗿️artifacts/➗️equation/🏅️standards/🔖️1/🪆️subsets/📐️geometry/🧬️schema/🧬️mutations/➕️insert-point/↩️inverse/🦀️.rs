//! ↩️ `insert-point` — undo is `remove-point` at the same (now FINAL-state) index, per the
//! index-keyed addressing law.

use crate::standards::v1::subsets::geometry::schema::mutations::remove_point;
use crate::EquationSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &super::InsertPoint, base: &EquationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::geometry::schema::mutations::remove_point::RemovePoint>,
{
    Ok((|| {
    let index = payload.index.min(base.geometry.points.len());
    vec![Op::from(remove_point::RemovePoint { index })]

    })())
}
//#endregion 🔖️Inverse
