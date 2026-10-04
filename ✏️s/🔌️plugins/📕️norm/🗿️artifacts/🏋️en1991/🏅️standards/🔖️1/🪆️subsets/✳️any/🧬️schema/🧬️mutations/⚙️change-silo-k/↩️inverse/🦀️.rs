//! Inverse for `change-silo-k`.
use super::ChangeSiloK;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeSiloK, base: &En1991Snapshot) -> Result<Vec<En1991Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1991Mutation::ChangeSiloK(ChangeSiloK { new_silo_k: base.silo_k })]

    })())
}
