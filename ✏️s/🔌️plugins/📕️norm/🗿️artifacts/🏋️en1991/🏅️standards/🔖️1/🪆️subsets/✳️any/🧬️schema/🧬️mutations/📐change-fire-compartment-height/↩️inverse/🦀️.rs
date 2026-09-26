//! Inverse for `change-fire-compartment-height`.
use super::ChangeFireCompartmentHeight;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeFireCompartmentHeight, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeFireCompartmentHeight(ChangeFireCompartmentHeight { new_fire_compartment_height: base.fire_compartment_height })]
}
