use crate::change_slope_angle::ChangeSlopeAngle;

use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &ChangeSlopeAngle, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_slope_angle::ChangeSlopeAngle> {
    Ok((|| {
    let a = base.slopes.iter().find(|f| f.id == payload.id).map(|f| f.angle_deg).unwrap_or(payload.new_angle_deg);
    vec![Op::from(ChangeSlopeAngle { id: payload.id.clone(), new_angle_deg: a })]

    })())
}
