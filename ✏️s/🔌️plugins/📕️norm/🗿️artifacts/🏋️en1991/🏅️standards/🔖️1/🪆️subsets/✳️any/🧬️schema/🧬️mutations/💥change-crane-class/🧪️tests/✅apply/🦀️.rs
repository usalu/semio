//! 💥 `change-crane-class` — upgrades the crane from class HC2 to HC3.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/💥change-crane-class/✅apply — the committed vector.

/// 💥 The committed `change-crane-class` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-crane-class",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/💥change-crane-class/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/💥change-crane-class/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/💥change-crane-class/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/💥change-crane-class/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/💥change-crane-class/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_crane_class_hc3() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-crane-class` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_crane_class_hc3_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}
