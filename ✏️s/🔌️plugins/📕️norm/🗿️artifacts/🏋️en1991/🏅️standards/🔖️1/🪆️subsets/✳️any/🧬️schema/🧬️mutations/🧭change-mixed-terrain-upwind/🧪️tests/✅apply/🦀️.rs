//! 🧭 `change-mixed-terrain-upwind` — smooths the upwind terrain of a mixed profile from category II to category I.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🧭change-mixed-terrain-upwind/✅apply — the committed vector.

/// 🧭 The committed `change-mixed-terrain-upwind` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-mixed-terrain-upwind",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭change-mixed-terrain-upwind/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭change-mixed-terrain-upwind/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭change-mixed-terrain-upwind/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭change-mixed-terrain-upwind/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭change-mixed-terrain-upwind/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_mixed_terrain_upwind_class_1() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-mixed-terrain-upwind` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_mixed_terrain_upwind_class_1_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}
