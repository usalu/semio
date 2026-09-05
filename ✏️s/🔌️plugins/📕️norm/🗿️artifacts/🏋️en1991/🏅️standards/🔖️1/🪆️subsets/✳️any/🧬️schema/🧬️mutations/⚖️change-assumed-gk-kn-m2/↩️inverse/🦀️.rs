//! ↩️ `change-assumed-gk-kn-m2` — undo restores BASE's assumed self-weight load.

use super::ChangeAssumedGKKnM2;
use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeAssumedGKKnM2, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeAssumedGKKnM2(ChangeAssumedGKKnM2 { new_assumed_g_k_kn_m2: base.assumed_g_k_kn_m2.clone() })]
}
//#endregion 🔖️Inverse
