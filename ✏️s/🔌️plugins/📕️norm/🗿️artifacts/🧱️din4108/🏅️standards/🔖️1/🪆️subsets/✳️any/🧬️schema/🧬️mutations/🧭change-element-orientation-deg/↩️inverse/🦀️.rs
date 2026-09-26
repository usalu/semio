//! Inverse for `change-element-orientation-deg`.

use super::ChangeElementOrientationDeg;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(_payload: &ChangeElementOrientationDeg, _base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    Vec::new()
}
