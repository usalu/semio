//! 🔺️ `update-bridge-inputs` — sparse diff construction.

use super::mutation::UpdateBridgeInputs;
use crate::artifacts::en1993::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateBridgeInputs, _base: &En1993Snapshot) -> En1993Diff {
    En1993Diff {
        bridge_lambda: Some(payload.new_bridge_lambda),
        bridge_phi_2: Some(payload.new_bridge_phi_2),
        bridge_delta_sigma_p_mpa: Some(payload.new_bridge_delta_sigma_p_mpa),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
