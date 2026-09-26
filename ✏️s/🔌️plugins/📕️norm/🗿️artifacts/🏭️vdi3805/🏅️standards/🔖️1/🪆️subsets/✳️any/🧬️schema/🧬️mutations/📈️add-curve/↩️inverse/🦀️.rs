//! ↩️ `add-curve` — undo is `remove-curve`, unless `base` already had this id (then `create`
//! was a no-op).

use super::AddCurve;
use crate::mutations::remove_curve;
use crate::{Vdi3805Mutation, Vdi3805Snapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &AddCurve, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
    if base.curves.contains_key(&payload.curve.id) {
        return Vec::new();
    }
    vec![Vdi3805Mutation::RemoveCurve(remove_curve::RemoveCurve { id: payload.curve.id.clone() })]
}
//#endregion 🔖️Inverse
