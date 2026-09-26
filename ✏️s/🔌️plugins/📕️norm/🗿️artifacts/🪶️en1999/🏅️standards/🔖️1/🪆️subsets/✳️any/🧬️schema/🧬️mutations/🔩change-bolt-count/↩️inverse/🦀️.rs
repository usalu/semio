//! ↩️ `change-bolt-count` inverse.

use crate::mutations::change_bolt_count::ChangeBoltCount;
use crate::mutations::En1999Mutation;
use crate::En1999Snapshot;

pub fn inverse(payload: &ChangeBoltCount, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    let (r, b) = base.connections.iter().find(|c| c.id == payload.connection_id).map(|c| (c.bolts.rows, c.bolts.bolts_per_row)).unwrap_or((1,1));
    vec![En1999Mutation::ChangeBoltCount(ChangeBoltCount { connection_id: payload.connection_id.clone(), new_rows: r, new_bolts_per_row: b })]
}
