//! Inverse for `change-zone-window-orientation`.

use super::ChangeZoneWindowOrientation;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(_payload: &ChangeZoneWindowOrientation, _base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    Vec::new()
}
