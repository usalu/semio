//! ↩️ Inverse for `change-t-max`.
use crate::change_t_max::ChangeTMax;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeTMax, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_t_max::ChangeTMax> {
    Ok((|| {
    vec![Op::from(ChangeTMax { new_t_max: base.t_max })]

    })())
}
