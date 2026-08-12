//! ↩️ `change-silo-height-m` — undo restores BASE's silo height.

use super::mutation::ChangeSiloHeightM;
use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeSiloHeightM, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeSiloHeightM(ChangeSiloHeightM { new_silo_height_m: base.silo_height_m.clone() })]
}
//#endregion 🔖️Inverse
