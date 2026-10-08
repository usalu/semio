//! 🧰 `change-construction-activity` — switches the execution-stage activity from scaffolding to formwork.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🔥change-construction-activity/✅apply — the committed vector.

/// 🧰 The committed `change-construction-activity` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-construction-activity",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔥change-construction-activity/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔥change-construction-activity/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔥change-construction-activity/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔥change-construction-activity/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔥change-construction-activity/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_construction_activity_formwork() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-construction-activity` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_construction_activity_formwork_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}
