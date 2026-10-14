//! Inverse for `change-t-min`.
use crate::change_t_min::ChangeTMin;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeTMin, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_t_min::ChangeTMin> {
    Ok((|| {
    vec![Op::from(ChangeTMin { new_t_min: base.t_min })]

    })())
}
