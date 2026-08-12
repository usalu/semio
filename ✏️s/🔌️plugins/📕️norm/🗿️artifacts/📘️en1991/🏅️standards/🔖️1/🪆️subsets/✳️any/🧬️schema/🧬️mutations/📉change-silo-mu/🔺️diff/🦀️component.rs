//! 🔺️ `change-silo-mu` — sparse diff construction.

use super::mutation::ChangeSiloMu;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeSiloMu, _base: &En1991Snapshot) -> En1991Diff {
    En1991Diff { silo_mu: Some(payload.new_silo_mu.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
