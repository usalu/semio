//! ↩️ `remove-thermal-bridge` inverse.

use super::RemoveThermalBridge;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(_payload: &RemoveThermalBridge, _base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    Vec::new()
}
