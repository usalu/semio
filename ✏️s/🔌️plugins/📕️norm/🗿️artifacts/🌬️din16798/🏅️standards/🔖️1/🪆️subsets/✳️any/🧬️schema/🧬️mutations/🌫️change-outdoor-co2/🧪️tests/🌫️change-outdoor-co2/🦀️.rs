//! 🧪 Named mutation test for `change-outdoor-co2`.

use crate::mutations::change_outdoor_co2;
use crate::{Din16798Mutation, Din16798Snapshot};
use protocol::MutationDiff;

#[semio_framework_async_macros::async_test]
async fn applies_change_outdoor_co2() {
    let base = Din16798Snapshot::default();
    let mutation = sample_mutation(&base);
    let outcome = <Din16798Mutation as protocol::Mutation<Din16798Snapshot>>::diff(&mutation, &base);
    assert_eq!(outcome.worst_level(), None, "change-outdoor-co2 should apply cleanly on the default subject");
    let after = MutationDiff::apply(outcome.diff(), &base).expect("applies");
    assert_ne!(after, base, "change-outdoor-co2 must change the snapshot");
    let inverse = <Din16798Mutation as protocol::Mutation<Din16798Snapshot>>::inverse(&mutation, &base);
    let mut restored = after;
    for step in &inverse {
        let undo = <Din16798Mutation as protocol::Mutation<Din16798Snapshot>>::diff(step, &restored);
        restored = MutationDiff::apply(undo.diff(), &restored).expect("inverse applies");
    }
    assert_eq!(restored, base, "change-outdoor-co2 inverse restores the base snapshot");
}

fn sample_mutation(base: &Din16798Snapshot) -> Din16798Mutation {
    Din16798Mutation::ChangeOutdoorCo2(change_outdoor_co2::ChangeOutdoorCo2 { new_outdoor_co2_ppm: base.outdoor_co2_ppm + 10.0 })
}
