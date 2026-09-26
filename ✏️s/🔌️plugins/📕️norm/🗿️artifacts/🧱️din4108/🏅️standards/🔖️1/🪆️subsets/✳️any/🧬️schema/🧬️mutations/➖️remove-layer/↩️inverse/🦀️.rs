//! ↩️ `remove-layer` inverse.

use super::RemoveLayer;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(_payload: &RemoveLayer, _base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    Vec::new()
}
