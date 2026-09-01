//! ↩️ `change-rh-int` — undo restores BASE's `rh_int`.

use super::ChangeRhInt;
use crate::artifacts::din4108::{Din4108Mutation, Din4108Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeRhInt, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    vec![Din4108Mutation::ChangeRhInt(ChangeRhInt { new_rh_int: base.rh_int })]
}
//#endregion 🔖️Inverse
