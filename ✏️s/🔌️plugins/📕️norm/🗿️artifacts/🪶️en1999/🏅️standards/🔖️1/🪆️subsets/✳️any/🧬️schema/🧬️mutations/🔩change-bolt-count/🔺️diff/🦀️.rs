//! 🔺️ `change-bolt-count` diff.

use crate::diff::En1999Diff;
use crate::mutations::change_bolt_count::ChangeBoltCount;
use crate::En1999Snapshot;

pub fn diff(payload: &ChangeBoltCount, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let mut connections = base.connections.clone();
    let Some(conn) = connections.iter_mut().find(|c| c.id == payload.connection_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Unknown connection {}", payload.connection_id), Vec::<String>::new());
    };
    conn.bolts.rows = payload.new_rows;
    conn.bolts.bolts_per_row = payload.new_bolts_per_row;
    protocol::MutationOutcome::new(En1999Diff { connections: Some(connections), ..Default::default() })
}
