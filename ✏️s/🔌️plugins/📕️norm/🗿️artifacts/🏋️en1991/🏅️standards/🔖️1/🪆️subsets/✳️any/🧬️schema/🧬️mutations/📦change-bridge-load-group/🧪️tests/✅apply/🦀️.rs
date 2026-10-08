//! 📦 `change-bridge-load-group` — switches the traffic load group from gr1a to gr1b.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/📦change-bridge-load-group/✅apply — the committed vector.

/// 📦 The committed `change-bridge-load-group` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-bridge-load-group",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/📦change-bridge-load-group/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/📦change-bridge-load-group/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/📦change-bridge-load-group/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/📦change-bridge-load-group/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/📦change-bridge-load-group/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_bridge_load_group_gr1b() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-bridge-load-group` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_bridge_load_group_gr1b_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}
