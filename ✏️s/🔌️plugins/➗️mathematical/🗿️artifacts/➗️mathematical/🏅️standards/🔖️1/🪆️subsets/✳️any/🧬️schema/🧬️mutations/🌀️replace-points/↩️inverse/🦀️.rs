//! ↩️ `replace-points` — undo reconstructed from BASE state (the whole prior point cloud).

use crate::artifacts::mathematical::{mathematical_geometry, MathematicalMutation, MathematicalSnapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::ReplacePoints, base: &MathematicalSnapshot) -> Vec<MathematicalMutation> {
    vec![MathematicalMutation::ReplacePoints(super::ReplacePoints { points: crate::artifacts::mathematical::mathematical_geometry(base).points })]
}
//#endregion 🔖️Inverse
