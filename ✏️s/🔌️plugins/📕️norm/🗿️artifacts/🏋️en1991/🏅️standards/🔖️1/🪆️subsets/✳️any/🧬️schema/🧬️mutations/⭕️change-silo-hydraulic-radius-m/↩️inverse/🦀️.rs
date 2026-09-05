//! ↩️ `change-silo-hydraulic-radius-m` — undo restores BASE's silo hydraulic radius.

use super::ChangeSiloHydraulicRadiusM;
use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeSiloHydraulicRadiusM, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeSiloHydraulicRadiusM(ChangeSiloHydraulicRadiusM { new_silo_hydraulic_radius_m: base.silo_hydraulic_radius_m.clone() })]
}
//#endregion 🔖️Inverse
