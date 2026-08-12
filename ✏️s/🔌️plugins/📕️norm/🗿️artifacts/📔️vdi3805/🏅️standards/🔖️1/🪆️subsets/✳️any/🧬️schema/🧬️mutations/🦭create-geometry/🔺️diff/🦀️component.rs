//! 🔺️ `create-geometry` — sparse diff construction.

use super::mutation::CreateGeometry;
use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Snapshot};

//#region 🔖️Diff
/// 🔺️ A duplicate id is a no-op — an id-keyed entity that already exists cannot be "created"
/// again; the map clone is returned unchanged rather than overwriting the existing entry.
pub fn diff(payload: &CreateGeometry, base: &Vdi3805Snapshot) -> Vdi3805Diff {
    let mut geometry = base.geometry.clone();
    if !geometry.contains_key(&payload.geometry.id) {
        geometry.insert(payload.geometry.id.clone(), payload.geometry.clone());
    }
    Vdi3805Diff { geometry: Some(geometry), ..Default::default() }
}
//#endregion 🔖️Diff
