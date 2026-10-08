//! 🚦 `change-assumed-bridge-udl` — assumes a 9 kPa uniformly distributed load UDL.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🛣️change-assumed-bridge-udl/✅apply — the committed vector.

/// 🚦 The committed `change-assumed-bridge-udl` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-assumed-bridge-udl",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🛣️change-assumed-bridge-udl/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🛣️change-assumed-bridge-udl/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🛣️change-assumed-bridge-udl/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🛣️change-assumed-bridge-udl/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🛣️change-assumed-bridge-udl/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_assumed_bridge_udl_9_kpa() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-assumed-bridge-udl` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_assumed_bridge_udl_9_kpa_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}
