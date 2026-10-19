//! ↩️ Inverse for `change-height`.
use crate::change_height::ChangeHeight;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeHeight, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_height::ChangeHeight> {
    Ok((|| {
    vec![Op::from(ChangeHeight { new_height: base.height })]

    })())
}
