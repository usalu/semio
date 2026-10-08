//! 🧱 `change-assumed-silo-wall-friction` — raises the assumed wall friction traction from 1 kPa to 2 kPa.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🧱change-assumed-silo-wall-friction/✅apply — the committed vector.

/// 🧱 The committed `change-assumed-silo-wall-friction` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-assumed-silo-wall-friction",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱change-assumed-silo-wall-friction/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱change-assumed-silo-wall-friction/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱change-assumed-silo-wall-friction/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱change-assumed-silo-wall-friction/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱change-assumed-silo-wall-friction/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_assumed_silo_wall_friction_2_kpa() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-assumed-silo-wall-friction` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_assumed_silo_wall_friction_2_kpa_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}
