//! 🔺️ `change-delta-sigma-mpa` — sparse diff construction.

use super::mutation::ChangeDeltaSigmaMpa;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeDeltaSigmaMpa, _base: &En1994Snapshot) -> En1994Diff {
    En1994Diff { delta_sigma_mpa: Some(payload.new_delta_sigma_mpa), ..Default::default() }
}
//#endregion 🔖️Diff
