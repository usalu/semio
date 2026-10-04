//! Inverse for `change-silo-hydraulic-radius`.
use super::ChangeSiloHydraulicRadius;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeSiloHydraulicRadius, base: &En1991Snapshot) -> Result<Vec<En1991Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1991Mutation::ChangeSiloHydraulicRadius(ChangeSiloHydraulicRadius { new_silo_hydraulic_radius: base.silo_hydraulic_radius })]

    })())
}
