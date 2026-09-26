//! 🧪️ `remove-footing` named test.

use crate::{En1997Mutation, En1997Snapshot};
use protocol::MutationDiff;

#[semio_framework_async_macros::async_test]
async fn applies_remove_footing() {
    let base = En1997Snapshot::default();
    let mutation = sample_mutation(&base);
    let outcome = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation, &base);
    assert_eq!(outcome.worst_level(), None, "remove-footing should apply cleanly on the default subject");
    let after = MutationDiff::apply(outcome.diff(), &base).expect("applies");
    assert_ne!(after, base, "remove-footing must change the snapshot");
    let inverse = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::inverse(&mutation, &base);
    let mut restored = after;
    for step in &inverse {
        let undo = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(step, &restored);
        restored = MutationDiff::apply(undo.diff(), &restored).expect("inverse applies");
    }
    assert_eq!(restored, base, "remove-footing inverse restores the base snapshot");
}

fn sample_mutation(base: &En1997Snapshot) -> En1997Mutation {
    use crate::mutations::*;
    En1997Mutation::RemoveFooting(remove_footing::RemoveFooting { index: 0 })
}
