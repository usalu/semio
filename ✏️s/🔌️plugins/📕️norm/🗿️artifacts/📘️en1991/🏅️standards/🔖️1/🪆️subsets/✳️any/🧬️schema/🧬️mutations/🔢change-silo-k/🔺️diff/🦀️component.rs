//! 🔺️ `change-silo-k` — sparse diff construction.

use super::mutation::ChangeSiloK;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeSiloK, _base: &En1991Snapshot) -> En1991Diff {
    En1991Diff { silo_k: Some(payload.new_silo_k.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
