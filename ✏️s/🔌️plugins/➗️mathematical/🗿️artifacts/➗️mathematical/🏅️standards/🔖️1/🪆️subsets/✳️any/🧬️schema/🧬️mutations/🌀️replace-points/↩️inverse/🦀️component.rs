//! ↩️ `replace-points` — undo reconstructed from BASE state (the whole prior point cloud).

use super::mutation::ReplacePoints;
use crate::artifacts::mathematical::{MathematicalMutation, MathematicalSnapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ReplacePoints, base: &MathematicalSnapshot) -> Vec<MathematicalMutation> {
    vec![MathematicalMutation::ReplacePoints(ReplacePoints { points: crate::artifacts::mathematical::mathematical_geometry(base).points })]
}
//#endregion 🔖️Inverse
