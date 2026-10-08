//! 🚦 `change-bridge-lane` — widens the carriageway from 1 to 3 notional lanes.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🌉change-bridge-lane/✅apply — the committed vector.

/// 🚦 The committed `change-bridge-lane` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-bridge-lane",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-bridge-lane/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-bridge-lane/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-bridge-lane/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-bridge-lane/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-bridge-lane/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_bridge_lane_3_lanes() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-bridge-lane` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_bridge_lane_3_lanes_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}
