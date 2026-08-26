//! ↩️ `change-silo-mu` — undo restores BASE's silo friction coefficient.

use super::mutation::ChangeSiloMu;
use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeSiloMu, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeSiloMu(ChangeSiloMu { new_silo_mu: base.silo_mu.clone() })]
}
//#endregion 🔖️Inverse
