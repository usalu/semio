//! ↩️ `add-curve` — undo is `remove-curve`, unless `base` already had this id (then `create`
//! was a no-op).

use crate::add_curve::AddCurve;
use crate::remove_curve;
use semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &AddCurve, base: &Vdi3805Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_curve::RemoveCurve> {
    Ok((|| {
    if base.curves.contains_key(&payload.curve.id) {
        return Vec::new();
    }
    vec![Op::from(remove_curve::RemoveCurve { id: payload.curve.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
