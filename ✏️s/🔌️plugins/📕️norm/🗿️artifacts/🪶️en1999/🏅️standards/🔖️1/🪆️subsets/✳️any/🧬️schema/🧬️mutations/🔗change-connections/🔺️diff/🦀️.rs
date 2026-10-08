//! 🔺️ `change-connections` diff.

use crate::mutations::change_connections::ChangeConnections;
use crate::En1999Snapshot;
use crate::diff::{En1999Diff, En1999ConnectionsRows};

pub fn diff(payload: &ChangeConnections, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if &base.connections == &payload.connections {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "List unchanged.");
    }
    if let Some((_, row)) = payload.connections.iter().enumerate().find(|(at, row)| payload.connections[..*at].iter().any(|earlier| earlier.id == row.id)) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Connection id {} appears twice.", row.id), [row.id.clone()]);
    }
    protocol::MutationOutcome::new(En1999Diff { connections: Some(En1999ConnectionsRows::setting(&base.connections, &payload.connections)), ..Default::default() })
}
