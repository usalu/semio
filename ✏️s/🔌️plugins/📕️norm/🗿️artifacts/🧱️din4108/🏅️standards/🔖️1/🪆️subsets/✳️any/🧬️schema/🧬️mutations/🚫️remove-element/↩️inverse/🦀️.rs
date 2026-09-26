//! ↩️ `remove-element` inverse.

use super::RemoveElement;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(_payload: &RemoveElement, _base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    Vec::new()
}
