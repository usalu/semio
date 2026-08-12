//! 🔺️ `add-geometry-connection` — sparse diff construction; upserts by `connection.id` (never
//! duplicates), missing geometry id is a no-op clone.

use super::mutation::AddGeometryConnection;
use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &AddGeometryConnection, base: &Vdi3805Snapshot) -> Vdi3805Diff {
    let mut geometry = base.geometry.clone();
    if let Some(entry) = geometry.get_mut(&payload.id) {
        entry.connections.retain(|c| c.id != payload.connection.id);
        entry.connections.push(payload.connection.clone());
    }
    Vdi3805Diff { geometry: Some(geometry), ..Default::default() }
}
//#endregion 🔖️Diff
