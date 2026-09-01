//! ↩️ `change-self-weight-thickness-m` — undo restores BASE's self-weight thickness.

use super::ChangeSelfWeightThicknessM;
use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeSelfWeightThicknessM, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeSelfWeightThicknessM(ChangeSelfWeightThicknessM { new_self_weight_thickness_m: base.self_weight_thickness_m.clone() })]
}
//#endregion 🔖️Inverse
