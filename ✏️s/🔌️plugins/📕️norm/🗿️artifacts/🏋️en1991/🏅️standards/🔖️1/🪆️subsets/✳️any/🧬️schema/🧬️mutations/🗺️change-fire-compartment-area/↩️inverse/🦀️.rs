//! Inverse for `change-fire-compartment-area`.
use super::ChangeFireCompartmentArea;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeFireCompartmentArea, base: &En1991Snapshot) -> Result<Vec<En1991Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1991Mutation::ChangeFireCompartmentArea(ChangeFireCompartmentArea { new_fire_compartment_area: base.fire_compartment_area })]

    })())
}
