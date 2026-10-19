//! ↩️ Inverse for `change-crane-claimed`.
use crate::change_crane_claimed::ChangeCraneClaimed;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeCraneClaimed, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_crane_claimed::ChangeCraneClaimed> {
    Ok((|| {
    vec![Op::from(ChangeCraneClaimed { new_crane_claimed: base.crane_claimed })]

    })())
}
