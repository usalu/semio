//! Inverse for `change-silo-bulk-density`.
use super::ChangeSiloBulkDensity;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeSiloBulkDensity, base: &En1991Snapshot) -> Result<Vec<En1991Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1991Mutation::ChangeSiloBulkDensity(ChangeSiloBulkDensity { new_silo_bulk_density: base.silo_bulk_density })]

    })())
}
