//! 🔺️ `change-bridge-moment-resistance-knm` — sparse diff construction.

use super::mutation::ChangeBridgeMomentResistanceKnm;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeBridgeMomentResistanceKnm, _base: &En1991Snapshot) -> En1991Diff {
    En1991Diff { bridge_moment_resistance_knm: Some(payload.new_bridge_moment_resistance_knm.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
