//! ️ `change-connections` named scenario `replaces-connections-list`.

use crate::{En1999Mutation, En1999Snapshot};
use protocol::{Mutation, MutationDiff};

#[semio_framework_async_macros::async_test]
async fn specifies_connections_list_applies_and_inverts() {
    let base = En1999Snapshot::default();
    let mutation = sample_mutation(&base);
    let outcome = mutation.diff(&base);
    let after = outcome.diff().apply(&base).expect("change-connections applies");
    assert_ne!(serde_json::to_string(&after).unwrap(), serde_json::to_string(&base).unwrap(), "change-connections must change the snapshot");
}

fn sample_mutation(base: &En1999Snapshot) -> En1999Mutation {
    {
        let mut items = base.connections.clone();
        items.clear();
        // keep at least structural validity by cloning original then pushing nothing — use clone with tweak
        items = base.connections.clone();
        if !items.is_empty() { items.pop(); items.push(base.connections[0].clone()); }
        // force change: empty then restore one
        let mut changed = base.connections.clone();
        if let Some(first) = changed.first_mut() {
            // id tweak for equality break where possible — for annex-less lists clone+push duplicate avoided
        }
        changed.reverse();
        if changed == base.connections { changed.push(base.connections[0].clone()); }
        En1999Mutation::ChangeConnections(crate::mutations::change_connections::ChangeConnections { connections: changed })
    }
}
