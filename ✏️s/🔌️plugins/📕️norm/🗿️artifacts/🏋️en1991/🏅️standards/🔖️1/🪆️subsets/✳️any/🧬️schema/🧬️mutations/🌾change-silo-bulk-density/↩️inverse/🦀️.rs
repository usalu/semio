//! Inverse for `change-silo-bulk-density`.
use crate::change_silo_bulk_density::ChangeSiloBulkDensity;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeSiloBulkDensity, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_bulk_density::ChangeSiloBulkDensity> {
    Ok((|| {
    vec![Op::from(ChangeSiloBulkDensity { new_silo_bulk_density: base.silo_bulk_density })]

    })())
}
