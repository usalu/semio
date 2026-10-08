//! 🌉 `change-bridge-span` — lengthens the bridge span from 20 m to 36 m.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🏗️change-bridge-span/✅apply — the committed vector.

/// 🌉 The committed `change-bridge-span` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-bridge-span",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏗️change-bridge-span/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏗️change-bridge-span/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏗️change-bridge-span/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏗️change-bridge-span/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏗️change-bridge-span/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_bridge_span_36_m() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-bridge-span` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_bridge_span_36_m_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}
