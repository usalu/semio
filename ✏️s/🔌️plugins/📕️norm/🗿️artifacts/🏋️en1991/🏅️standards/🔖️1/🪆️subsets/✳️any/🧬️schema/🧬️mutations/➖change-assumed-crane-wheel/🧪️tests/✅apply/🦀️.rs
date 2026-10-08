//! 🛞 `change-assumed-crane-wheel` — assumes a 75 kN crane wheel load.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/➖change-assumed-crane-wheel/✅apply — the committed vector.

/// 🛞 The committed `change-assumed-crane-wheel` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-assumed-crane-wheel",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖change-assumed-crane-wheel/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖change-assumed-crane-wheel/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖change-assumed-crane-wheel/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖change-assumed-crane-wheel/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖change-assumed-crane-wheel/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_assumed_crane_wheel_75_kn() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-assumed-crane-wheel` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_assumed_crane_wheel_75_kn_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}
