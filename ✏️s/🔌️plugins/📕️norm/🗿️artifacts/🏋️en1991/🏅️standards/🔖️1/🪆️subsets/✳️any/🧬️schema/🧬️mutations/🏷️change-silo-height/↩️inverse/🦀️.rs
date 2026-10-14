//! Inverse for `change-silo-height`.
use crate::change_silo_height::ChangeSiloHeight;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeSiloHeight, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_height::ChangeSiloHeight> {
    Ok((|| {
    vec![Op::from(ChangeSiloHeight { new_silo_height: base.silo_height })]

    })())
}
