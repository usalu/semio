//! Inverse for `change-air-density`.
use super::ChangeAirDensity;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeAirDensity, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeAirDensity(ChangeAirDensity { new_air_density: base.air_density })]
}
