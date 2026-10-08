//! 🪝 `change-hoist-class` — upgrades the hoist from class HC2 to HC4.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/➕change-hoist-class/✅apply — the committed vector.

/// 🪝 The committed `change-hoist-class` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-hoist-class",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕change-hoist-class/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕change-hoist-class/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕change-hoist-class/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕change-hoist-class/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕change-hoist-class/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_hoist_class_hc4() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-hoist-class` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_hoist_class_hc4_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}
