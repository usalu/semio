//! Inverse for `change-altitude`.
use super::ChangeAltitude;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeAltitude, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeAltitude(ChangeAltitude { new_altitude: base.altitude })]
}
