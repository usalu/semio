//! 🚶 `change-assumed-bridge-footway` — assumes a 5 kPa footway load.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🚶change-assumed-bridge-footway/✅apply — the committed vector.

/// 🚶 The committed `change-assumed-bridge-footway` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-assumed-bridge-footway",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚶change-assumed-bridge-footway/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚶change-assumed-bridge-footway/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚶change-assumed-bridge-footway/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚶change-assumed-bridge-footway/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚶change-assumed-bridge-footway/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_assumed_bridge_footway_5_kpa() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-assumed-bridge-footway` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_assumed_bridge_footway_5_kpa_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}
