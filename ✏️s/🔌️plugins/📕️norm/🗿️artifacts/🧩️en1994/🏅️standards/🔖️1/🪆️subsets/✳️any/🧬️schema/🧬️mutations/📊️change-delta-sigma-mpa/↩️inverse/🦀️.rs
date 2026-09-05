//! ↩️ `change-delta-sigma-mpa` — undo restores BASE's delta_sigma_mpa.

use super::ChangeDeltaSigmaMpa;
use crate::artifacts::en1994::{En1994Mutation, En1994Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeDeltaSigmaMpa, base: &En1994Snapshot) -> Vec<En1994Mutation> {
    vec![En1994Mutation::ChangeDeltaSigmaMpa(ChangeDeltaSigmaMpa { new_delta_sigma_mpa: base.delta_sigma_mpa })]
}
//#endregion 🔖️Inverse
