//! 🔺️ `change-silo-height-m` — sparse diff construction.

use super::mutation::ChangeSiloHeightM;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeSiloHeightM, _base: &En1991Snapshot) -> En1991Diff {
    En1991Diff { silo_height_m: Some(payload.new_silo_height_m.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
