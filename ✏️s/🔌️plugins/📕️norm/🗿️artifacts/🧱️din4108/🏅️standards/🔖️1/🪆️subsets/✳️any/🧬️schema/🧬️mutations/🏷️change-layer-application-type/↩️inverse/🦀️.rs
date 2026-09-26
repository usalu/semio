//! Inverse for `change-layer-application-type`.

use super::ChangeLayerApplicationType;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(_payload: &ChangeLayerApplicationType, _base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    Vec::new()
}
