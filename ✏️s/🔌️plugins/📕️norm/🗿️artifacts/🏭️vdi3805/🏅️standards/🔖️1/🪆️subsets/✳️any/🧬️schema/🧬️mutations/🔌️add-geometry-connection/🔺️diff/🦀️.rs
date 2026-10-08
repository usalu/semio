//! 🔺️ `add-geometry-connection` — sparse diff construction; upserts by `connection.id` (never duplicates) and inserts at `index`
//! (absent appends, out of range clamps with `mutation.clamped`); missing geometry id is `mutation.target-missing`, an identical
//! existing connection is `mutation.no-op`.

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
    let replaced = entry.connections.iter().any(|connection| connection.id == payload.connection.id);
    let removed = if replaced { vec![payload.connection.id.clone()] } else { Vec::new() };
    let survivors: Vec<String> = entry.connections.iter().filter(|connection| connection.id != payload.connection.id).map(|connection| connection.id.clone()).collect();
    let clamped = matches!(payload.index, Some(index) if index > survivors.len());
    let at = payload.index.filter(|index| *index <= survivors.len()).unwrap_or(survivors.len());
    let order = (at < survivors.len()).then(|| {
        let mut order = survivors.clone();
        order.insert(at, payload.connection.id.clone());
        order
    });
    let outcome = protocol::MutationOutcome::new(Vdi3805Diff {
        geometry: Some(Vdi3805GeometryRows {
            modified: vec![Vdi3805GeometryPatch {
                key: payload.id.clone(),
                connections: Some(Vdi3805GeometryConnectionsRows { added: vec![payload.connection.clone()], removed, order, ..Default::default() }),
                ..Default::default()
            }],
            ..Default::default()
        }),
        ..Default::default()
    });
    if clamped {
        outcome.warning("mutation.clamped", format!("Insert index was out of range; appended connection \"{}\" at the end instead.", payload.connection.id))
    } else {
        outcome
    }
}
//#endregion 🔖️Diff
