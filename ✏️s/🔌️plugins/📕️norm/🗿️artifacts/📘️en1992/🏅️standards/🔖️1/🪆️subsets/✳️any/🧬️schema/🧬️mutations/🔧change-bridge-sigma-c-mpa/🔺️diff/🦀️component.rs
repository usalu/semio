//! 🔺️ `change-bridge-sigma-c-mpa` sparse diff construction — writes only `En1992Diff.bridge_sigma_c_mpa` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_bridge_sigma_c_mpa::mutation::ChangeBridgeSigmaCMpa;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeBridgeSigmaCMpa, _base: &En1992Snapshot) -> En1992Diff {
    En1992Diff { bridge_sigma_c_mpa: Some(payload.new_bridge_sigma_c_mpa.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
