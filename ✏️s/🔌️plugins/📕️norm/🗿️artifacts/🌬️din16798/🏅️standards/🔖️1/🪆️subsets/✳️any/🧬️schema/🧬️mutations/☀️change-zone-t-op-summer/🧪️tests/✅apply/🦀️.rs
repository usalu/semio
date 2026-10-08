//! 🧪 Named mutation test for `change-zone-t-op-summer`.

use crate::mutations::change_zone_t_op_summer;
use crate::{Din16798Mutation, Din16798Snapshot};

#[semio_framework_async_macros::async_test]
async fn applies_change_zone_t_op_summer() {
    let base = Din16798Snapshot::default();
    let mutation = sample_mutation(&base);
    let outcome = <Din16798Mutation as protocol::Mutation<Din16798Snapshot>>::diff(&mutation, &base);
    assert_eq!(outcome.worst_level(), None, "change-zone-t-op-summer should apply cleanly on the default subject");
    let after = protocol::apply_diff(outcome.diff(), &base).expect("applies");
    assert_ne!(after, base, "change-zone-t-op-summer must change the snapshot");
    let inverse = <Din16798Mutation as protocol::Mutation<Din16798Snapshot>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture");
    let mut restored = after;
    for step in &inverse {
        let undo = <Din16798Mutation as protocol::Mutation<Din16798Snapshot>>::diff(step, &restored);
        restored = protocol::apply_diff(undo.diff(), &restored).expect("inverse applies");
    }
    assert_eq!(restored, base, "change-zone-t-op-summer inverse restores the base snapshot");
}

fn sample_mutation(base: &Din16798Snapshot) -> Din16798Mutation {
    Din16798Mutation::ChangeZoneTOpSummer(change_zone_t_op_summer::ChangeZoneTOpSummer { zone_id: base.zones[0].id.clone(), new_t_op_summer_c: base.zones[0].t_op_summer_c + 0.5 })
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = Din16798Snapshot::default();
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&sample_mutation(&base), &base).await;
}
