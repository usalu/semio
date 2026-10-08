//! 🚛 `change-assumed-bridge-lm3` — assumes a 600 kN special vehicle LM3.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🚛change-assumed-bridge-lm3/✅apply — the committed vector.

/// 🚛 The committed `change-assumed-bridge-lm3` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-assumed-bridge-lm3",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚛change-assumed-bridge-lm3/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚛change-assumed-bridge-lm3/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚛change-assumed-bridge-lm3/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚛change-assumed-bridge-lm3/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚛change-assumed-bridge-lm3/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_assumed_bridge_lm3_600_kn() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-assumed-bridge-lm3` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_assumed_bridge_lm3_600_kn_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}
