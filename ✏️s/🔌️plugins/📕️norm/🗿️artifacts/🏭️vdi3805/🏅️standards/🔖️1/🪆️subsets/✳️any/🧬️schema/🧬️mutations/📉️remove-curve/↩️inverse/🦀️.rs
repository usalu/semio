//! ↩️ `remove-curve` — undo re-`create`s the curve from BASE state; missing id ⇒ `Vec::new()`.

use crate::remove_curve::RemoveCurve;
use crate::add_curve;
use semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &RemoveCurve, base: &Vdi3805Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::add_curve::AddCurve> {
    Ok((|| {
    match base.curves.get(&payload.id) {
        Some(curve) => vec![Op::from(add_curve::AddCurve { curve: curve.clone() })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
