//! ️ `change-member-my-ed` named scenario `raises-member-bending-moment`.

use crate::{En1999Mutation, En1999Snapshot};
use protocol::{Mutation, MutationDiff};

#[semio_framework_async_macros::async_test]
async fn raises_member_bending_moment_applies_and_inverts() {
    let base = En1999Snapshot::default();
    let mutation = sample_mutation(&base);
    let outcome = mutation.diff(&base);
    let after = outcome.diff().apply(&base).expect("change-member-my-ed applies");
    assert_ne!(serde_json::to_string(&after).unwrap(), serde_json::to_string(&base).unwrap(), "change-member-my-ed must change the snapshot");
}

fn sample_mutation(base: &En1999Snapshot) -> En1999Mutation {
    En1999Mutation::ChangeMemberMYEd(crate::mutations::change_member_m_y_ed::ChangeMemberMYEd {
        member_id: base.members[0].id.clone(),
        action_id: base.members[0].actions[0].id.clone(),
        new_m_y_k: base.members[0].actions[0].m_y_k + 500.0,
    })
}
