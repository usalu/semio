//! Inverse for `change-layer-compressive-class`.

use super::ChangeLayerCompressiveClass;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(_payload: &ChangeLayerCompressiveClass, _base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    Vec::new()
}
