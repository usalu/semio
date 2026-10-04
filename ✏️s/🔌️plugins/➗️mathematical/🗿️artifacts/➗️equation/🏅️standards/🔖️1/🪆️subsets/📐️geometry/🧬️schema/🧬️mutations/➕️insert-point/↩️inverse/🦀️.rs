//! ↩️ `insert-point` — undo is `remove-point` at the same (now FINAL-state) index, per the
//! index-keyed addressing law.

use crate::standards::v1::subsets::geometry::schema::mutations::remove_point;
use crate::{EquationMutation, EquationSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &super::InsertPoint, base: &EquationSnapshot) -> Result<Vec<EquationMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let index = payload.index.min(base.geometry.points.len());
    vec![EquationMutation::RemovePoint(remove_point::RemovePoint { index })]

    })())
}
//#endregion 🔖️Inverse
