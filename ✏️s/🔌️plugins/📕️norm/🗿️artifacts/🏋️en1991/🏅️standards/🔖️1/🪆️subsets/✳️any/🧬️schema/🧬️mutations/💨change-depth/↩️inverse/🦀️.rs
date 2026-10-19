//! ↩️ Inverse for `change-depth`.
use crate::change_depth::ChangeDepth;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeDepth, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_depth::ChangeDepth> {
    Ok((|| {
    vec![Op::from(ChangeDepth { new_depth: base.depth })]

    })())
}
