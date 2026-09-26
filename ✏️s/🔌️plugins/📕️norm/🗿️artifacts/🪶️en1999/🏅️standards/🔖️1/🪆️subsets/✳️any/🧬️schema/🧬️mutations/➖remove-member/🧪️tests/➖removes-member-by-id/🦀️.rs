//! ️ `remove-member` named scenario `removes-member-by-id`.

use crate::{En1999Mutation, En1999Snapshot};
use protocol::{Mutation, MutationDiff};

#[semio_framework_async_macros::async_test]
async fn drops_member_by_id_applies_and_inverts() {
    let base = En1999Snapshot::default();
    let mutation = sample_mutation(&base);
    let outcome = mutation.diff(&base);
    let after = outcome.diff().apply(&base).expect("remove-member applies");
    assert_ne!(serde_json::to_string(&after).unwrap(), serde_json::to_string(&base).unwrap(), "remove-member must change the snapshot");
}

fn sample_mutation(base: &En1999Snapshot) -> En1999Mutation {
    En1999Mutation::RemoveMember(crate::mutations::remove_member::RemoveMember { id: base.members[0].id.clone() })
}
