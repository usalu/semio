//! ↩️ Inverse for `change-hoist-class`.
use crate::change_hoist_class::ChangeHoistClass;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeHoistClass, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_hoist_class::ChangeHoistClass> {
    Ok((|| {
    vec![Op::from(ChangeHoistClass { new_hoist_class: base.hoist_class.clone() })]

    })())
}
