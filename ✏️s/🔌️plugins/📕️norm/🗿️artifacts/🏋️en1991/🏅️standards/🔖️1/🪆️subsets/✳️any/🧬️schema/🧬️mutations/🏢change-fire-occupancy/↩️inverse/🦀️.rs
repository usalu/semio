//! Inverse for `change-fire-occupancy`.
use super::ChangeFireOccupancy;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeFireOccupancy, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeFireOccupancy(ChangeFireOccupancy { new_fire_occupancy: base.fire_occupancy.clone() })]
}
