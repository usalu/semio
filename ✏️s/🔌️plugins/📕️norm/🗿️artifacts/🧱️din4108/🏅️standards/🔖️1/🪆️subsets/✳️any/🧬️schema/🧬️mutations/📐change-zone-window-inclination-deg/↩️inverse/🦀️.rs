//! Inverse for `change-zone-window-inclination-deg`.

use super::ChangeZoneWindowInclinationDeg;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(_payload: &ChangeZoneWindowInclinationDeg, _base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    Vec::new()
}
