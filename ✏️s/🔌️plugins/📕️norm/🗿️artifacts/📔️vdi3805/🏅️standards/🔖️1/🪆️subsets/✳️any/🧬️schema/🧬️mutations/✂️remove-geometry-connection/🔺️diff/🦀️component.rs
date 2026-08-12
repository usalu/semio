//! 🔺️ `remove-geometry-connection` — sparse diff construction.

use super::mutation::RemoveGeometryConnection;
use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &RemoveGeometryConnection, base: &Vdi3805Snapshot) -> Vdi3805Diff {
    let mut geometry = base.geometry.clone();
    if let Some(entry) = geometry.get_mut(&payload.id) {
        entry.connections.retain(|c| c.id != payload.connection_id);
    }
    Vdi3805Diff { geometry: Some(geometry), ..Default::default() }
}
//#endregion 🔖️Diff
