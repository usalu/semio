//! ↩️ `replace-points` — undo reconstructed from BASE state (the whole prior point cloud).

use crate::{EquationMutation, EquationSnapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ReplacePoints, base: &EquationSnapshot) -> Result<Vec<EquationMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![EquationMutation::ReplacePoints(super::ReplacePoints { points: base.geometry.points.clone() })]

    })())
}
//#endregion 🔖️Inverse
