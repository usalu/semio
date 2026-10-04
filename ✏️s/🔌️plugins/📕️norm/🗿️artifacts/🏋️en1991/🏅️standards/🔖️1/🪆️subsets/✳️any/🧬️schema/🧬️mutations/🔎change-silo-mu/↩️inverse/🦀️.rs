//! Inverse for `change-silo-mu`.
use super::ChangeSiloMu;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeSiloMu, base: &En1991Snapshot) -> Result<Vec<En1991Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1991Mutation::ChangeSiloMu(ChangeSiloMu { new_silo_mu: base.silo_mu })]

    })())
}
