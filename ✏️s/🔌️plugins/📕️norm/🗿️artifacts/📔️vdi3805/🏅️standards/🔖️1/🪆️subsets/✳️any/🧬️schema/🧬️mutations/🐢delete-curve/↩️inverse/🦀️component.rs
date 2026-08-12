//! ↩️ `delete-curve` — undo re-`create`s the curve from BASE state; missing id ⇒ `Vec::new()`.

use super::mutation::DeleteCurve;
use crate::artifacts::vdi3805::mutations::create_curve;
use crate::artifacts::vdi3805::{Vdi3805Mutation, Vdi3805Snapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteCurve, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
    match base.curves.get(&payload.id) {
        Some(curve) => vec![Vdi3805Mutation::CreateCurve(create_curve::mutation::CreateCurve { curve: curve.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
