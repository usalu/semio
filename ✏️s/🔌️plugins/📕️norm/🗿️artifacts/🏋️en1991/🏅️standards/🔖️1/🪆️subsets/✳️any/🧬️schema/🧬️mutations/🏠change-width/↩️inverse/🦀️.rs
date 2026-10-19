//! ↩️ Inverse for `change-width`.
use crate::change_width::ChangeWidth;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeWidth, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_width::ChangeWidth> {
    Ok((|| {
    vec![Op::from(ChangeWidth { new_width: base.width })]

    })())
}
