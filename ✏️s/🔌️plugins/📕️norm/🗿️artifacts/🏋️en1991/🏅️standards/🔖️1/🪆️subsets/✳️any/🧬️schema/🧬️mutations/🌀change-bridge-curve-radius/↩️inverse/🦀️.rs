//! ↩️ Inverse for `change-bridge-curve-radius`.
use crate::change_bridge_curve_radius::ChangeBridgeCurveRadius;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeBridgeCurveRadius, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_curve_radius::ChangeBridgeCurveRadius> {
    Ok((|| {
    vec![Op::from(ChangeBridgeCurveRadius { new_bridge_curve_radius: base.bridge_curve_radius })]

    })())
}
