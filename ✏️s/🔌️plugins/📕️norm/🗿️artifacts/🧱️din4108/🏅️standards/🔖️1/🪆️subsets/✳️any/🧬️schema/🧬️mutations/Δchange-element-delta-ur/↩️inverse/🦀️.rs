//! Inverse for `change-element-delta-ur`.

use super::ChangeElementDeltaUr;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(_payload: &ChangeElementDeltaUr, _base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    Vec::new()
}
