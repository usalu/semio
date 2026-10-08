//! 🧱 `change-fire-thermal-inertia` — raises the enclosure thermal inertia b from 1160 to 1500 J/(m²s^½K).
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🧱change-fire-thermal-inertia/✅apply — the committed vector.

/// 🧱 The committed `change-fire-thermal-inertia` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-fire-thermal-inertia",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱change-fire-thermal-inertia/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱change-fire-thermal-inertia/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱change-fire-thermal-inertia/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱change-fire-thermal-inertia/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱change-fire-thermal-inertia/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_fire_thermal_inertia_1500() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-fire-thermal-inertia` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_fire_thermal_inertia_1500_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}
