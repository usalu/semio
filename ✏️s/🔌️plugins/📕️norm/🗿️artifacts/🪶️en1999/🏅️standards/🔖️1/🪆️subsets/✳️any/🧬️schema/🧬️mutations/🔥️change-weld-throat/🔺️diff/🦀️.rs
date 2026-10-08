//! 🔺️ `change-weld-throat` diff.

use crate::mutations::change_weld_throat::ChangeWeldThroat;
use crate::En1999Snapshot;
use crate::diff::{En1999Diff, En1999ConnectionsRows, En1999ConnectionsPatch};

pub fn diff(payload: &ChangeWeldThroat, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !base.connections.iter().any(|connection| connection.id == payload.connection_id) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Unknown connection {}", payload.connection_id), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1999Diff {
        connections: Some(En1999ConnectionsRows { modified: vec![En1999ConnectionsPatch { id: payload.connection_id.clone(), welds_throat: Some(payload.new_throat), ..Default::default() }], ..Default::default() }),
        ..Default::default()
    })
}
