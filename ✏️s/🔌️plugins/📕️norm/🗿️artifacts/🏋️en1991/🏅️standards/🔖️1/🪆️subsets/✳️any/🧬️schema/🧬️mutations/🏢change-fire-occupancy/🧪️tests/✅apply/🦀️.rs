//! 🏬 `change-fire-occupancy` — reclassifies the fire occupancy from office to shopping.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🏢change-fire-occupancy/✅apply — the committed vector.

/// 🏬 The committed `change-fire-occupancy` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-fire-occupancy",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏢change-fire-occupancy/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏢change-fire-occupancy/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏢change-fire-occupancy/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏢change-fire-occupancy/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏢change-fire-occupancy/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_fire_occupancy_shopping() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-fire-occupancy` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_fire_occupancy_shopping_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}
