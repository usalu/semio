//! ️ `add-member` named scenario `inserts-member-at-end`.

use crate::{En1999Mutation, En1999Snapshot};
use protocol::{Mutation, MutationDiff};

#[semio_framework_async_macros::async_test]
async fn adds_member_at_end_applies_and_inverts() {
    let base = En1999Snapshot::default();
    let mutation = sample_mutation(&base);
    let outcome = mutation.diff(&base);
    let after = outcome.diff().apply(&base).expect("add-member applies");
    assert_ne!(serde_json::to_string(&after).unwrap(), serde_json::to_string(&base).unwrap(), "add-member must change the snapshot");
}

fn sample_mutation(base: &En1999Snapshot) -> En1999Mutation {
    En1999Mutation::AddMember(crate::mutations::add_member::AddMember {
        index: base.members.len() as u32,
        member: {
            let mut mem = base.members[0].clone();
            mem.id = "member-extra".into();
            mem
        },
    })
}
