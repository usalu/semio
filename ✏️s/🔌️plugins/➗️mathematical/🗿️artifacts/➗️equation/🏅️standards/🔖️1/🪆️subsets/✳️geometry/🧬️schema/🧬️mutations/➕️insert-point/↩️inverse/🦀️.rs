//! ↩️ `insert-point` — undo is `remove-point` at the same (now FINAL-state) index, per the
//! index-keyed addressing law.

use crate::artifacts::equation::standards::v1::subsets::geometry::schema::mutations::remove_point;
use crate::artifacts::equation::{equation_geometry, EquationMutation, EquationSnapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &super::InsertPoint, base: &EquationSnapshot) -> Vec<EquationMutation> {
    let index = payload.index.min(crate::artifacts::equation::equation_geometry(base).points.len());
    vec![EquationMutation::RemovePoint(remove_point::RemovePoint { index })]
}
//#endregion 🔖️Inverse
