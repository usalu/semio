//! ↩️ `change-curve-points` — undo restores BASE's points; missing id ⇒ `Vec::new()`.

use crate::change_curve_points::ChangeCurvePoints;
use semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &ChangeCurvePoints, base: &Vdi3805Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_curve_points::ChangeCurvePoints> {
    Ok((|| {
    let Some(curve) = base.curves.get(&payload.id) else {
        return Vec::new();
    };
    vec![Op::from(ChangeCurvePoints { id: payload.id.clone(), new_points: curve.points.clone() })]

    })())
}
//#endregion 🔖️Inverse
