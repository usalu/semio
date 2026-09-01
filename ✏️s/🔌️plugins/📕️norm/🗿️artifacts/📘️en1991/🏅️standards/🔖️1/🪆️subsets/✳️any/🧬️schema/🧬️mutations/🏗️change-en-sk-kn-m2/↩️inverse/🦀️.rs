//! ↩️ `change-en-sk-kn-m2` — undo restores BASE's characteristic snow load.

use super::ChangeEnSKKnM2;
use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeEnSKKnM2, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeEnSKKnM2(ChangeEnSKKnM2 { new_en_s_k_kn_m2: base.en_s_k_kn_m2.clone() })]
}
//#endregion 🔖️Inverse
