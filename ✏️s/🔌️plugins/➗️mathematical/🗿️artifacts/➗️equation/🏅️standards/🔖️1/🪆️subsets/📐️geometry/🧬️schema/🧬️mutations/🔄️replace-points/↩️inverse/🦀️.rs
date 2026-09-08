//! ↩️ `replace-points` — undo reconstructed from BASE state (the whole prior point cloud).

use crate::artifacts::equation::{EquationMutation, EquationSnapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ReplacePoints, base: &EquationSnapshot) -> Vec<EquationMutation> {
    vec![EquationMutation::ReplacePoints(super::ReplacePoints { points: crate::artifacts::equation::equation_geometry(base).points })]
}
//#endregion 🔖️Inverse
