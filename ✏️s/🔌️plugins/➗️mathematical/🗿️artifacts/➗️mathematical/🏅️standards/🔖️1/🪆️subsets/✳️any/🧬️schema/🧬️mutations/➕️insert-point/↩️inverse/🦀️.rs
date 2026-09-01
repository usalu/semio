//! ↩️ `insert-point` — undo is `remove-point` at the same (now FINAL-state) index, per the
//! index-keyed addressing law.

use crate::artifacts::mathematical::mutations::remove_point;
use crate::artifacts::mathematical::{mathematical_geometry, MathematicalMutation, MathematicalSnapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &super::InsertPoint, base: &MathematicalSnapshot) -> Vec<MathematicalMutation> {
    let index = payload.index.min(crate::artifacts::mathematical::mathematical_geometry(base).points.len());
    vec![MathematicalMutation::RemovePoint(remove_point::RemovePoint { index })]
}
//#endregion 🔖️Inverse
