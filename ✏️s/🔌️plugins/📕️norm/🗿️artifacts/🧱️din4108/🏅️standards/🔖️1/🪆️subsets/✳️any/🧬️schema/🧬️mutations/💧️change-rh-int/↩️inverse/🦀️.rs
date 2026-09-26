//! ↩️ `change-rh-int` inverse.

use super::ChangeRhInt;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(_payload: &ChangeRhInt, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    vec![Din4108Mutation::ChangeRhInt(ChangeRhInt { new_rh_int: base.rh_int })]
}
