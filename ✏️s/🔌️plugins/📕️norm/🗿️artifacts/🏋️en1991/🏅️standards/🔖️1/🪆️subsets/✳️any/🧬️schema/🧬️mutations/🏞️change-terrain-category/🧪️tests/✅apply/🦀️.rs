//! 🌳 `change-terrain-category` — roughens the terrain from category II to category III.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🏞️change-terrain-category/✅apply — the committed vector.

/// 🌳 The committed `change-terrain-category` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-terrain-category",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏞️change-terrain-category/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏞️change-terrain-category/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏞️change-terrain-category/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏞️change-terrain-category/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏞️change-terrain-category/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_terrain_category_class_3() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-terrain-category` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_terrain_category_class_3_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}
