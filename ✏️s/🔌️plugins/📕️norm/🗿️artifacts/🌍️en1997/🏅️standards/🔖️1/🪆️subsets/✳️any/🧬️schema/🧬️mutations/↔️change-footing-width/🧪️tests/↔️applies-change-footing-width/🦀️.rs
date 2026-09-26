//! 🧪️ `change-footing-width` named test.

use crate::{En1997Mutation, En1997Snapshot};
use protocol::MutationDiff;

#[semio_framework_async_macros::async_test]
async fn applies_change_footing_width() {
    let base = En1997Snapshot::default();
    let mutation = sample_mutation(&base);
    let outcome = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation, &base);
    assert_eq!(outcome.worst_level(), None, "change-footing-width should apply cleanly on the default subject");
    let after = MutationDiff::apply(outcome.diff(), &base).expect("applies");
    assert_ne!(after, base, "change-footing-width must change the snapshot");
    let inverse = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::inverse(&mutation, &base);
    let mut restored = after;
    for step in &inverse {
        let undo = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(step, &restored);
        restored = MutationDiff::apply(undo.diff(), &restored).expect("inverse applies");
    }
    assert_eq!(restored, base, "change-footing-width inverse restores the base snapshot");
}

fn sample_mutation(base: &En1997Snapshot) -> En1997Mutation {
    use crate::mutations::*;
    En1997Mutation::ChangeFootingWidth(change_footing_width::ChangeFootingWidth { id: base.footings[0].id.clone(), new_width: base.footings[0].width + 0.5 })
}
