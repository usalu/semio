//! ↩️ Inverse for `change-crane-class`.
use crate::change_crane_class::ChangeCraneClass;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeCraneClass, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_crane_class::ChangeCraneClass> {
    Ok((|| {
    vec![Op::from(ChangeCraneClass { new_crane_class: base.crane_class.clone() })]

    })())
}
