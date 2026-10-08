//! 📏 `change-mixed-terrain-distance` — moves the upwind terrain change from 0 m to 1500 m.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/📏change-mixed-terrain-distance/✅apply — the committed vector.

/// 📏 The committed `change-mixed-terrain-distance` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-mixed-terrain-distance",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/📏change-mixed-terrain-distance/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/📏change-mixed-terrain-distance/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/📏change-mixed-terrain-distance/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/📏change-mixed-terrain-distance/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/📏change-mixed-terrain-distance/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_mixed_terrain_distance_1500_m() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-mixed-terrain-distance` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_mixed_terrain_distance_1500_m_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}
