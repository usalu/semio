//! 🔺️ `change-bridge-delta-sigma-s-mpa` sparse diff construction — writes only `En1992Diff.bridge_delta_sigma_s_mpa` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_bridge_delta_sigma_s_mpa::mutation::ChangeBridgeDeltaSigmaSMpa;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeBridgeDeltaSigmaSMpa, _base: &En1992Snapshot) -> En1992Diff {
    En1992Diff { bridge_delta_sigma_s_mpa: Some(payload.new_bridge_delta_sigma_s_mpa.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
