//! Inverse for `change-element-delta-uf`.

use super::ChangeElementDeltaUf;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(_payload: &ChangeElementDeltaUf, _base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    Vec::new()
}
