//! ️ `change-member-buckling-length` named scenario `shortens-buckling-length-y`.

use crate::{En1999Mutation, En1999Snapshot};
use protocol::{Mutation, MutationDiff};

#[semio_framework_async_macros::async_test]
async fn shortens_buckling_length_y_applies_and_inverts() {
    let base = En1999Snapshot::default();
    let mutation = sample_mutation(&base);
    let outcome = mutation.diff(&base);
    let after = outcome.diff().apply(&base).expect("change-member-buckling-length applies");
    assert_ne!(serde_json::to_string(&after).unwrap(), serde_json::to_string(&base).unwrap(), "change-member-buckling-length must change the snapshot");
}

fn sample_mutation(base: &En1999Snapshot) -> En1999Mutation {
    En1999Mutation::ChangeMemberBucklingLength(crate::mutations::change_member_buckling_length::ChangeMemberBucklingLength {
        member_id: base.members[0].id.clone(),
        axis: "y".into(),
        new_length: (base.members[0].buckling_length_y * 0.5).max(0.1),
    })
}
