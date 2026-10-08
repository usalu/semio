//! 🌉 `change-structure-kind` — reclassifies the structure from a building to a bridge.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🌉change-structure-kind/✅apply — the committed vector.

/// 🌉 The committed `change-structure-kind` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-structure-kind",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-structure-kind/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-structure-kind/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-structure-kind/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-structure-kind/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-structure-kind/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_structure_kind_bridge() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-structure-kind` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_structure_kind_bridge_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}
