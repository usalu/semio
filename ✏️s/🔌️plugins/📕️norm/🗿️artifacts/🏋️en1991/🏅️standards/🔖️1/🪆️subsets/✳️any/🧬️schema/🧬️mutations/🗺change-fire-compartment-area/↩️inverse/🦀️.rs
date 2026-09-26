//! Inverse for `change-fire-compartment-area`.
use super::ChangeFireCompartmentArea;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeFireCompartmentArea, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeFireCompartmentArea(ChangeFireCompartmentArea { new_fire_compartment_area: base.fire_compartment_area })]
}
