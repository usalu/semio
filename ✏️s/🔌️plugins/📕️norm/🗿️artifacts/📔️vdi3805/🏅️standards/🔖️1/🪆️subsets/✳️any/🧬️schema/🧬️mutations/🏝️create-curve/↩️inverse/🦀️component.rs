//! ↩️ `create-curve` — undo is `delete-curve`, unless `base` already had this id (then `create`
//! was a no-op).

use super::mutation::CreateCurve;
use crate::artifacts::vdi3805::mutations::delete_curve;
use crate::artifacts::vdi3805::{Vdi3805Mutation, Vdi3805Snapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &CreateCurve, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
    if base.curves.contains_key(&payload.curve.id) {
        return Vec::new();
    }
    vec![Vdi3805Mutation::DeleteCurve(delete_curve::mutation::DeleteCurve { id: payload.curve.id.clone() })]
}
//#endregion 🔖️Inverse
