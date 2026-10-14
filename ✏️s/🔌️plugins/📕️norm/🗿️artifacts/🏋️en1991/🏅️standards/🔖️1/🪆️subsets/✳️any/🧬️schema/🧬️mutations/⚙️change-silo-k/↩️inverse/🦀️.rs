//! Inverse for `change-silo-k`.
use crate::change_silo_k::ChangeSiloK;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeSiloK, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_k::ChangeSiloK> {
    Ok((|| {
    vec![Op::from(ChangeSiloK { new_silo_k: base.silo_k })]

    })())
}
