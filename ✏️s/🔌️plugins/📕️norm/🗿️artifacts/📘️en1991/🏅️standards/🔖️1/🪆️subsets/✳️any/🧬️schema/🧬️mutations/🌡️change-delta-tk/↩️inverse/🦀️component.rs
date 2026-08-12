//! ↩️ `change-delta-tk` — undo restores BASE's thermal delta.

use super::mutation::ChangeDeltaTK;
use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeDeltaTK, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeDeltaTK(ChangeDeltaTK { new_delta_t_k: base.delta_t_k.clone() })]
}
//#endregion 🔖️Inverse
