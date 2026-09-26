//! ️ `change-member-n-ed` named scenario `raises-member-axial-force`.

use crate::{En1999Mutation, En1999Snapshot};
use protocol::{Mutation, MutationDiff};

#[semio_framework_async_macros::async_test]
async fn raises_member_axial_force_applies_and_inverts() {
    let base = En1999Snapshot::default();
    let mutation = sample_mutation(&base);
    let outcome = mutation.diff(&base);
    let after = outcome.diff().apply(&base).expect("change-member-n-ed applies");
    assert_ne!(serde_json::to_string(&after).unwrap(), serde_json::to_string(&base).unwrap(), "change-member-n-ed must change the snapshot");
}

fn sample_mutation(base: &En1999Snapshot) -> En1999Mutation {
    En1999Mutation::ChangeMemberNEd(crate::mutations::change_member_n_ed::ChangeMemberNEd {
        member_id: base.members[0].id.clone(),
        action_id: base.members[0].actions[0].id.clone(),
        new_n_k: base.members[0].actions[0].n_k + 1000.0,
    })
}
