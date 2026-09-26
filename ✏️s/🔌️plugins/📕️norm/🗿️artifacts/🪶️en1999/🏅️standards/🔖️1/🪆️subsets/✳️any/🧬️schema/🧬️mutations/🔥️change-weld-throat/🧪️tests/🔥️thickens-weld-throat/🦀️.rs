//! ️ `change-weld-throat` named scenario `thickens-weld-throat`.

use crate::{En1999Mutation, En1999Snapshot};
use protocol::{Mutation, MutationDiff};

#[semio_framework_async_macros::async_test]
async fn thickens_weld_throat_applies_and_inverts() {
    let base = En1999Snapshot::default();
    let mutation = sample_mutation(&base);
    let outcome = mutation.diff(&base);
    let after = outcome.diff().apply(&base).expect("change-weld-throat applies");
    assert_ne!(serde_json::to_string(&after).unwrap(), serde_json::to_string(&base).unwrap(), "change-weld-throat must change the snapshot");
}

fn sample_mutation(base: &En1999Snapshot) -> En1999Mutation {
    En1999Mutation::ChangeWeldThroat(crate::mutations::change_weld_throat::ChangeWeldThroat {
        connection_id: base.connections[0].id.clone(),
        new_throat: base.connections[0].welds.throat + 0.001,
    })
}
