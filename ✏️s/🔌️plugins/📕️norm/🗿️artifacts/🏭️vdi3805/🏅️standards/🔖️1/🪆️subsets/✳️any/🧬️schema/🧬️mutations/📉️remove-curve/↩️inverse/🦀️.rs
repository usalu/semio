//! ↩️ `remove-curve` — undo re-`create`s the curve from BASE state; missing id ⇒ `Vec::new()`.

use super::RemoveCurve;
use crate::mutations::add_curve;
use crate::{Vdi3805Mutation, Vdi3805Snapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &RemoveCurve, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
    match base.curves.get(&payload.id) {
        Some(curve) => vec![Vdi3805Mutation::AddCurve(add_curve::AddCurve { curve: curve.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
