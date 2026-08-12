//! 🔺️ `change-silo-hydraulic-radius-m` — sparse diff construction.

use super::mutation::ChangeSiloHydraulicRadiusM;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeSiloHydraulicRadiusM, _base: &En1991Snapshot) -> En1991Diff {
    En1991Diff { silo_hydraulic_radius_m: Some(payload.new_silo_hydraulic_radius_m.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
