//! Inverse for `change-fire-curve`.
use crate::change_fire_curve::ChangeFireCurve;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeFireCurve, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fire_curve::ChangeFireCurve> {
    Ok((|| {
    vec![Op::from(ChangeFireCurve { new_fire_curve: base.fire_curve })]

    })())
}
