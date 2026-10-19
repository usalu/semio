//! ↩️ Inverse for `change-en-vb`.
use crate::change_en_vb::ChangeEnVb;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeEnVb, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_en_vb::ChangeEnVb> {
    Ok((|| {
    vec![Op::from(ChangeEnVb { new_en_vb: base.en_vb })]

    })())
}
