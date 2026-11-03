//! ↩️ Inverse for `change-fire-load-basis`.
use crate::change_fire_load_basis::ChangeFireLoadBasis;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeFireLoadBasis, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fire_load_basis::ChangeFireLoadBasis> {
    Ok((|| {
    vec![Op::from(ChangeFireLoadBasis { new_fire_load_basis: base.fire_load_basis })]

    })())
}
