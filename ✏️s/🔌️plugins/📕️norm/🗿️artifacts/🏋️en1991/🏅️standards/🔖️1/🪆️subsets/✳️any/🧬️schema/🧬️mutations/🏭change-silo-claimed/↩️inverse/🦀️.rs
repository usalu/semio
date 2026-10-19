//! ↩️ Inverse for `change-silo-claimed`.
use crate::change_silo_claimed::ChangeSiloClaimed;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeSiloClaimed, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_claimed::ChangeSiloClaimed> {
    Ok((|| {
    vec![Op::from(ChangeSiloClaimed { new_silo_claimed: base.silo_claimed })]

    })())
}
