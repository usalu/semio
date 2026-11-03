//! ↩️ Inverse for `change-crane-x`.
use crate::change_crane_x::ChangeCraneX;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeCraneX, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_crane_x::ChangeCraneX> {
    Ok((|| {
    vec![Op::from(ChangeCraneX { new_crane_x: base.crane_x })]

    })())
}
