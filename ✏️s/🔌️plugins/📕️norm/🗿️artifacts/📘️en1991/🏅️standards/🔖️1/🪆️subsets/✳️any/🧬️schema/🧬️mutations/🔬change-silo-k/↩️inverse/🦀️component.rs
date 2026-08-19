//! ↩️ `change-silo-k` — undo restores BASE's silo lateral pressure ratio.

use super::mutation::ChangeSiloK;
use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeSiloK, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeSiloK(ChangeSiloK { new_silo_k: base.silo_k.clone() })]
}
//#endregion 🔖️Inverse
