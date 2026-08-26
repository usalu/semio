//! ↩️ `change-self-weight-material` — undo restores BASE's self-weight material.

use super::mutation::ChangeSelfWeightMaterial;
use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeSelfWeightMaterial, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeSelfWeightMaterial(ChangeSelfWeightMaterial { new_self_weight_material: base.self_weight_material.clone() })]
}
//#endregion 🔖️Inverse
