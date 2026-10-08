//! 📏 `change-bridge-lane-width` — widens the notional lane from 3 m to 3.5 m.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/↔️change-bridge-lane-width/✅apply — the committed vector.

/// 📏 The committed `change-bridge-lane-width` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-bridge-lane-width",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/↔️change-bridge-lane-width/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/↔️change-bridge-lane-width/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/↔️change-bridge-lane-width/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/↔️change-bridge-lane-width/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/↔️change-bridge-lane-width/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_bridge_lane_width_3_5_m() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-bridge-lane-width` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_bridge_lane_width_3_5_m_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}
