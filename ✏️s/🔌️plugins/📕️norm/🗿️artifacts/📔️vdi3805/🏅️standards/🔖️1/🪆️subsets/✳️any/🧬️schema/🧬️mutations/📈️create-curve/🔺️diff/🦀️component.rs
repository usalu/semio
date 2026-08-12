//! 🔺️ `create-curve` — sparse diff construction.

use super::mutation::CreateCurve;
use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Snapshot};

//#region 🔖️Diff
/// 🔺️ A duplicate id is a no-op — an id-keyed entity that already exists cannot be "created"
/// again; the map clone is returned unchanged rather than overwriting the existing entry.
pub fn diff(payload: &CreateCurve, base: &Vdi3805Snapshot) -> Vdi3805Diff {
    let mut curves = base.curves.clone();
    if !curves.contains_key(&payload.curve.id) {
        curves.insert(payload.curve.id.clone(), payload.curve.clone());
    }
    Vdi3805Diff { curves: Some(curves), ..Default::default() }
}
//#endregion 🔖️Diff
