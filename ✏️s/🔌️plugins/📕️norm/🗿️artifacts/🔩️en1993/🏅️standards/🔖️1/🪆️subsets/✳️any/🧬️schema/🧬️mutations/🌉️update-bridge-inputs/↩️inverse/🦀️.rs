//! ↩️ `update-bridge-inputs` — undo restores BASE's bridge inputs.

use super::UpdateBridgeInputs;
use crate::artifacts::en1993::{En1993Mutation, En1993Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &UpdateBridgeInputs, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    vec![En1993Mutation::UpdateBridgeInputs(UpdateBridgeInputs { new_bridge_lambda: base.bridge_lambda, new_bridge_phi_2: base.bridge_phi_2, new_bridge_delta_sigma_p_mpa: base.bridge_delta_sigma_p_mpa })]
}
//#endregion 🔖️Inverse
