//! Inverse for `change-element-inclination-deg`.

use super::ChangeElementInclinationDeg;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(_payload: &ChangeElementInclinationDeg, _base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    Vec::new()
}
