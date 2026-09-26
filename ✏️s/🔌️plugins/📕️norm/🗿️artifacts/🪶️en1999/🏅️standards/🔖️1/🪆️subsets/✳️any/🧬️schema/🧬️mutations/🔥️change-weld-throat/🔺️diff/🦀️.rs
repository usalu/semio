//! 🔺️ `change-weld-throat` diff.

use crate::diff::En1999Diff;
use crate::mutations::change_weld_throat::ChangeWeldThroat;
use crate::En1999Snapshot;

pub fn diff(payload: &ChangeWeldThroat, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let mut connections = base.connections.clone();
    let Some(conn) = connections.iter_mut().find(|c| c.id == payload.connection_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Unknown connection {}", payload.connection_id), Vec::<String>::new());
    };
    conn.welds.throat = payload.new_throat;
    protocol::MutationOutcome::new(En1999Diff { connections: Some(connections), ..Default::default() })
}
