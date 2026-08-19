//! ↩️ `change-bridge-moment-resistance-knm` — undo restores BASE's bridge moment resistance.

use super::mutation::ChangeBridgeMomentResistanceKnm;
use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeBridgeMomentResistanceKnm, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeBridgeMomentResistanceKnm(ChangeBridgeMomentResistanceKnm { new_bridge_moment_resistance_knm: base.bridge_moment_resistance_knm.clone() })]
}
//#endregion 🔖️Inverse
