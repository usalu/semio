//! 💧 `change-silo-kind` — reclassifies the container from a silo to a tank.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/⚖️change-silo-kind/✅apply — the committed vector.

/// 💧 The committed `change-silo-kind` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-silo-kind",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/⚖️change-silo-kind/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/⚖️change-silo-kind/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/⚖️change-silo-kind/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/⚖️change-silo-kind/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/⚖️change-silo-kind/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_silo_kind_tank() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-silo-kind` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_silo_kind_tank_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}
