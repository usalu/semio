//! 🔺️ `change-silo-bulk-density-kn-m3` — sparse diff construction.

use super::mutation::ChangeSiloBulkDensityKnM3;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeSiloBulkDensityKnM3, _base: &En1991Snapshot) -> En1991Diff {
    En1991Diff { silo_bulk_density_kn_m3: Some(payload.new_silo_bulk_density_kn_m3.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
