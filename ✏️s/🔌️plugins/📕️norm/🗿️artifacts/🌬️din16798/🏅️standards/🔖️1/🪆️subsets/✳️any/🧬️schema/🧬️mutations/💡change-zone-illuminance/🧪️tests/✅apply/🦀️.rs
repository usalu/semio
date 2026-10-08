//! 🧪 Named mutation test for `change-zone-illuminance`.

use crate::mutations::change_zone_illuminance;
use crate::{Din16798Mutation, Din16798Snapshot};

#[semio_framework_async_macros::async_test]
async fn applies_change_zone_illuminance() {
    let base = Din16798Snapshot::default();
    let mutation = sample_mutation(&base);
    let outcome = <Din16798Mutation as protocol::Mutation<Din16798Snapshot>>::diff(&mutation, &base);
    assert_eq!(outcome.worst_level(), None, "change-zone-illuminance should apply cleanly on the default subject");
    let after = protocol::apply_diff(outcome.diff(), &base).expect("applies");
    assert_ne!(after, base, "change-zone-illuminance must change the snapshot");
    let inverse = <Din16798Mutation as protocol::Mutation<Din16798Snapshot>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture");
    let mut restored = after;
    for step in &inverse {
        let undo = <Din16798Mutation as protocol::Mutation<Din16798Snapshot>>::diff(step, &restored);
        restored = protocol::apply_diff(undo.diff(), &restored).expect("inverse applies");
    }
    assert_eq!(restored, base, "change-zone-illuminance inverse restores the base snapshot");
}

fn sample_mutation(base: &Din16798Snapshot) -> Din16798Mutation {
    Din16798Mutation::ChangeZoneIlluminance(change_zone_illuminance::ChangeZoneIlluminance { zone_id: base.zones[0].id.clone(), new_illuminance_lx: base.zones[0].illuminance_lx + 10.0 })
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = Din16798Snapshot::default();
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&sample_mutation(&base), &base).await;
}
