//! 🔺️ `add-geometry-connection` — sparse diff construction; upserts by `connection.id` (never
//! duplicates), missing geometry id is `mutation.target-missing`, an identical existing connection
//! is `mutation.no-op`; an explicit `index` past the end is `mutation.target-missing`.

use super::AddGeometryConnection;
use crate::{Vdi3805Snapshot};
use crate::diff::{Vdi3805Diff, Vdi3805GeometryRows, Vdi3805GeometryPatch, Vdi3805GeometryConnectionsRows};

//#region 🔖️Diff

pub fn diff(payload: &AddGeometryConnection, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    let Some(entry) = base.geometry.get(&payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Geometry \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if entry.connections.contains(&payload.connection) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Connection \"{}\" already exists on geometry \"{}\".", payload.connection.id, payload.id));
    }
    let survivors = entry.connections.iter().filter(|connection| connection.id != payload.connection.id).count();
    if let Some(index) = payload.index.filter(|index| *index > survivors) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {index} is past the end ({survivors} rows) for \"{}\".", payload.connection.id), Vec::<String>::new());
    }
    let mut connections = Vdi3805GeometryConnectionsRows::insertion(payload.index.unwrap_or(survivors), payload.connection.clone());
    if let Some(position) = entry.connections.iter().position(|connection| connection.id == payload.connection.id) {
        connections.removed = Vdi3805GeometryConnectionsRows::removal(&entry.connections, position).removed;
    }
    protocol::MutationOutcome::new(Vdi3805Diff {
        geometry: Some(Vdi3805GeometryRows {
            modified: vec![Vdi3805GeometryPatch { key: payload.id.clone(), connections: Some(connections), ..Default::default() }],
            ..Default::default()
        }),
        ..Default::default()
    })
}
