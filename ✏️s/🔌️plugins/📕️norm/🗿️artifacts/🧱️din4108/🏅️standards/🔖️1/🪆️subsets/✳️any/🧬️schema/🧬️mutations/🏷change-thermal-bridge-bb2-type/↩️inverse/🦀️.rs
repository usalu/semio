//! Inverse for `change-thermal-bridge-bb2-type`.

use super::ChangeThermalBridgeBb2Type;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(_payload: &ChangeThermalBridgeBb2Type, _base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    Vec::new()
}
