//! ↩️ `change-climate` — undo restores BASE's `climate`.

use super::mutation::ChangeClimate;
use crate::artifacts::din4108::{Din4108Mutation, Din4108Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeClimate, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    vec![Din4108Mutation::ChangeClimate(ChangeClimate { new_climate: base.climate })]
}
//#endregion 🔖️Inverse
