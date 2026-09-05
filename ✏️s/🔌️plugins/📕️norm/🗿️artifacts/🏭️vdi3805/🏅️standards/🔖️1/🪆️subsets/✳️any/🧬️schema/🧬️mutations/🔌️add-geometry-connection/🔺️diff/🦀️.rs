//! 🔺️ `add-geometry-connection` — sparse diff construction; upserts by `connection.id` (never
//! duplicates), missing geometry id is `mutation.target-missing`, an identical existing connection
//! is `mutation.no-op`.

use super::AddGeometryConnection;
use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &AddGeometryConnection, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    let Some(entry) = base.geometry.get(&payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Geometry \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if entry.connections.iter().any(|c| *c == payload.connection) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Connection \"{}\" already exists on geometry \"{}\".", payload.connection.id, payload.id));
    }
    let mut geometry = base.geometry.clone();
    if let Some(entry) = geometry.get_mut(&payload.id) {
        entry.connections.retain(|c| c.id != payload.connection.id);
        entry.connections.push(payload.connection.clone());
    }
    protocol::MutationOutcome::new(Vdi3805Diff { geometry: Some(geometry), ..Default::default() })
}
//#endregion 🔖️Diff
