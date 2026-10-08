//! 🏢 `change-storey-count` — raises the storey count from 3 to 5.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🏙️change-storey-count/✅apply — the committed vector.

/// 🏢 The committed `change-storey-count` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-storey-count",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏙️change-storey-count/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏙️change-storey-count/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏙️change-storey-count/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏙️change-storey-count/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏙️change-storey-count/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_storey_count_5_storeys() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-storey-count` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_storey_count_5_storeys_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}
