//! 🔺️ `change-en-sk-kn-m2` — sparse diff construction.

use super::mutation::ChangeEnSKKnM2;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeEnSKKnM2, _base: &En1991Snapshot) -> En1991Diff {
    En1991Diff { en_s_k_kn_m2: Some(payload.new_en_s_k_kn_m2.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
