//! ↩️ `change-silo-bulk-density-kn-m3` — undo restores BASE's silo bulk density.

use super::mutation::ChangeSiloBulkDensityKnM3;
use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeSiloBulkDensityKnM3, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeSiloBulkDensityKnM3(ChangeSiloBulkDensityKnM3 { new_silo_bulk_density_kn_m3: base.silo_bulk_density_kn_m3.clone() })]
}
//#endregion 🔖️Inverse
