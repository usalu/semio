//! ⌛ `change-fire-duration` — extends the fire duration from 60 min to 90 min.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/⏱️change-fire-duration/✅apply — the committed vector.

/// ⌛ The committed `change-fire-duration` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-fire-duration",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/⏱️change-fire-duration/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/⏱️change-fire-duration/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/⏱️change-fire-duration/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/⏱️change-fire-duration/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/⏱️change-fire-duration/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_fire_duration_90_min() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-fire-duration` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_fire_duration_90_min_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}
