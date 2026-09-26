//! 🔺️ `change-connections` diff.

use crate::diff::En1999Diff;
use crate::mutations::change_connections::ChangeConnections;
use crate::En1999Snapshot;

pub fn diff(payload: &ChangeConnections, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if &base.connections == &payload.connections {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "List unchanged.");
    }
    protocol::MutationOutcome::new(En1999Diff { connections: Some(payload.connections.clone()), ..Default::default() })
}
