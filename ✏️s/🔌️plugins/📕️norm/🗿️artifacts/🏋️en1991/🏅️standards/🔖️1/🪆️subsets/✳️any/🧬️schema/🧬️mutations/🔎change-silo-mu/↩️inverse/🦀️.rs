//! Inverse for `change-silo-mu`.
use crate::change_silo_mu::ChangeSiloMu;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeSiloMu, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_mu::ChangeSiloMu> {
    Ok((|| {
    vec![Op::from(ChangeSiloMu { new_silo_mu: base.silo_mu })]

    })())
}
