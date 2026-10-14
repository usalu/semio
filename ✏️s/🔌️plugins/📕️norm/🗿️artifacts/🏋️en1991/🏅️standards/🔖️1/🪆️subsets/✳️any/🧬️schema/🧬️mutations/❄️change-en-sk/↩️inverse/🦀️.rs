//! Inverse for `change-en-sk`.
use crate::change_en_sk::ChangeEnSk;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeEnSk, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_en_sk::ChangeEnSk> {
    Ok((|| {
    vec![Op::from(ChangeEnSk { new_en_sk: base.en_sk })]

    })())
}
