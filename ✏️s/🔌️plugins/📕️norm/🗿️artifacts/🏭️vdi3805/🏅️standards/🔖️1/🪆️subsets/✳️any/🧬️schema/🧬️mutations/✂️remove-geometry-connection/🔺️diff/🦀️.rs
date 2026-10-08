//! 🔺️ `remove-geometry-connection` — sparse diff construction.

use super::RemoveGeometryConnection;
use crate::{Vdi3805Snapshot};
use crate::diff::{Vdi3805Diff, Vdi3805GeometryRows, Vdi3805GeometryPatch, Vdi3805GeometryConnectionsRows};

//#region 🔖️Diff

pub fn diff(payload: &RemoveGeometryConnection, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    let Some(entry) = base.geometry.get(&payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Geometry \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let Some(index) = entry.connections.iter().position(|c| c.id == payload.connection_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Connection \"{}\" does not exist on geometry \"{}\".", payload.connection_id, payload.id), [payload.id.clone(), payload.connection_id.clone()]);
    };
    protocol::MutationOutcome::new(Vdi3805Diff {
        geometry: Some(Vdi3805GeometryRows {
            modified: vec![Vdi3805GeometryPatch {
                key: payload.id.clone(),
                connections: Some(Vdi3805GeometryConnectionsRows::removal(&entry.connections, index)),
                ..Default::default()
            }],
            ..Default::default()
        }),
        ..Default::default()
    })
}
