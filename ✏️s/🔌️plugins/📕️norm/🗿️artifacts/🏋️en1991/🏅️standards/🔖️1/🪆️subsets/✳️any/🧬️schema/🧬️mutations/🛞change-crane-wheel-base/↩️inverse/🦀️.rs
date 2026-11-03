//! ↩️ Inverse for `change-crane-wheel-base`.
use crate::change_crane_wheel_base::ChangeCraneWheelBase;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeCraneWheelBase, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_crane_wheel_base::ChangeCraneWheelBase> {
    Ok((|| {
    vec![Op::from(ChangeCraneWheelBase { new_crane_wheel_base: base.crane_wheel_base })]

    })())
}
