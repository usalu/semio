//! Inverse for `change-element-delta-ug`.

use super::ChangeElementDeltaUg;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(_payload: &ChangeElementDeltaUg, _base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    Vec::new()
}
