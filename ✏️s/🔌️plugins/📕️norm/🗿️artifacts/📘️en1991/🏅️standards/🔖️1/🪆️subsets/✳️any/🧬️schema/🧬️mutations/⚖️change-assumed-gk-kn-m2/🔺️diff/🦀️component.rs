//! 🔺️ `change-assumed-gk-kn-m2` — sparse diff construction.

use super::mutation::ChangeAssumedGKKnM2;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeAssumedGKKnM2, _base: &En1991Snapshot) -> En1991Diff {
    En1991Diff { assumed_g_k_kn_m2: Some(payload.new_assumed_g_k_kn_m2.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
