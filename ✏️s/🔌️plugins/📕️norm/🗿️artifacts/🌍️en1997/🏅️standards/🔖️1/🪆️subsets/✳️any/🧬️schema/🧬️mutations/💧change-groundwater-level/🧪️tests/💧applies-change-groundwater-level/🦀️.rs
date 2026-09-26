//! 🧪️ `change-groundwater-level` named test.

use crate::{En1997Mutation, En1997Snapshot};
use protocol::MutationDiff;

#[semio_framework_async_macros::async_test]
async fn applies_change_groundwater_level() {
    let base = En1997Snapshot::default();
    let mutation = sample_mutation(&base);
    let outcome = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation, &base);
    assert_eq!(outcome.worst_level(), None, "change-groundwater-level should apply cleanly on the default subject");
    let after = MutationDiff::apply(outcome.diff(), &base).expect("applies");
    assert_ne!(after, base, "change-groundwater-level must change the snapshot");
    let inverse = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::inverse(&mutation, &base);
    let mut restored = after;
    for step in &inverse {
        let undo = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(step, &restored);
        restored = MutationDiff::apply(undo.diff(), &restored).expect("inverse applies");
    }
    assert_eq!(restored, base, "change-groundwater-level inverse restores the base snapshot");
}

fn sample_mutation(base: &En1997Snapshot) -> En1997Mutation {
    use crate::mutations::*;
    En1997Mutation::ChangeGroundwaterLevel(change_groundwater_level::ChangeGroundwaterLevel { new_groundwater_level: 5.5 })
}
