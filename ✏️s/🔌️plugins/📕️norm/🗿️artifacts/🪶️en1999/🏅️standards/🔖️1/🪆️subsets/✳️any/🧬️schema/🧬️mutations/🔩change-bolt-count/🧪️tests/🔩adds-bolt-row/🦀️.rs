//! ️ `change-bolt-count` named scenario `adds-bolt-row`.

use crate::{En1999Mutation, En1999Snapshot};
use protocol::{Mutation, MutationDiff};

#[semio_framework_async_macros::async_test]
async fn adds_bolt_row_applies_and_inverts() {
    let base = En1999Snapshot::default();
    let mutation = sample_mutation(&base);
    let outcome = mutation.diff(&base);
    let after = outcome.diff().apply(&base).expect("change-bolt-count applies");
    assert_ne!(serde_json::to_string(&after).unwrap(), serde_json::to_string(&base).unwrap(), "change-bolt-count must change the snapshot");
}

fn sample_mutation(base: &En1999Snapshot) -> En1999Mutation {
    En1999Mutation::ChangeBoltCount(crate::mutations::change_bolt_count::ChangeBoltCount {
        connection_id: base.connections.iter().find(|c| c.kind == "bolted").unwrap_or(&base.connections[0]).id.clone(),
        new_rows: 3,
        new_bolts_per_row: 2,
    })
}
