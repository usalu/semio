//! 🔺️ `remove-geometry-connection` — sparse diff construction.

use super::mutation::RemoveGeometryConnection;
use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &RemoveGeometryConnection, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    let Some(entry) = base.geometry.get(&payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Geometry \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if !entry.connections.iter().any(|c| c.id == payload.connection_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Connection \"{}\" does not exist on geometry \"{}\".", payload.connection_id, payload.id), [payload.id.clone(), payload.connection_id.clone()]);
    }
    let mut geometry = base.geometry.clone();
    if let Some(entry) = geometry.get_mut(&payload.id) {
        entry.connections.retain(|c| c.id != payload.connection_id);
    }
    protocol::MutationOutcome::new(Vdi3805Diff { geometry: Some(geometry), ..Default::default() })
}
//#endregion 🔖️Diff
