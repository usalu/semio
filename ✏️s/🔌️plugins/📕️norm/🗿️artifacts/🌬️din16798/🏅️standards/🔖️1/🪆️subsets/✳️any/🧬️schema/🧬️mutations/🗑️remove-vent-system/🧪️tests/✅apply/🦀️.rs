//! 🧪 Named mutation test for `remove-vent-system`.

use crate::mutations::remove_vent_system;
use crate::{Din16798Mutation, Din16798Snapshot};

#[semio_framework_async_macros::async_test]
async fn applies_remove_vent_system() {
    let base = Din16798Snapshot::default();
    let mutation = sample_mutation(&base);
    let outcome = <Din16798Mutation as protocol::Mutation<Din16798Snapshot>>::diff(&mutation, &base);
    assert_eq!(outcome.worst_level(), None, "remove-vent-system should apply cleanly on the default subject");
    let after = protocol::apply_diff(outcome.diff(), &base).expect("applies");
    assert_ne!(after, base, "remove-vent-system must change the snapshot");
    let inverse = <Din16798Mutation as protocol::Mutation<Din16798Snapshot>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture");
    let mut restored = after;
    for step in &inverse {
        let undo = <Din16798Mutation as protocol::Mutation<Din16798Snapshot>>::diff(step, &restored);
        restored = protocol::apply_diff(undo.diff(), &restored).expect("inverse applies");
    }
    assert_eq!(restored, base, "remove-vent-system inverse restores the base snapshot");
}

fn sample_mutation(base: &Din16798Snapshot) -> Din16798Mutation {
    Din16798Mutation::RemoveVentSystem(remove_vent_system::RemoveVentSystem { vent_id: base.vent_systems[0].id.clone() })
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = Din16798Snapshot::default();
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&sample_mutation(&base), &base).await;
}
