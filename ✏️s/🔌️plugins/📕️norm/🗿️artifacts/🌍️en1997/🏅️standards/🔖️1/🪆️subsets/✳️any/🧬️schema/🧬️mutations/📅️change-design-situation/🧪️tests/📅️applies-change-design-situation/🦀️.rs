//! 🧪️ `change-design-situation` named test.

use crate::{En1997Mutation, En1997Snapshot};
use protocol::MutationDiff;

#[semio_framework_async_macros::async_test]
async fn applies_change_design_situation() {
    let base = En1997Snapshot::default();
    let mutation = sample_mutation(&base);
    let outcome = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation, &base);
    assert_eq!(outcome.worst_level(), None, "change-design-situation should apply cleanly on the default subject");
    let after = MutationDiff::apply(outcome.diff(), &base).expect("applies");
    assert_ne!(after, base, "change-design-situation must change the snapshot");
    let inverse = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::inverse(&mutation, &base);
    let mut restored = after;
    for step in &inverse {
        let undo = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(step, &restored);
        restored = MutationDiff::apply(undo.diff(), &restored).expect("inverse applies");
    }
    assert_eq!(restored, base, "change-design-situation inverse restores the base snapshot");
}

fn sample_mutation(base: &En1997Snapshot) -> En1997Mutation {
    use crate::mutations::*;
    En1997Mutation::ChangeDesignSituation(change_design_situation::ChangeDesignSituation { new_design_situation: "bsT".into() })
}
