//! 🌍 `change-annex` — switches the national annex from the German NA to the recommended EN values.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🌍change-annex/✅apply — the committed vector.

/// 🌍 The committed `change-annex` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-annex",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌍change-annex/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌍change-annex/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌍change-annex/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌍change-annex/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌍change-annex/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_annex_en() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-annex` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_annex_en_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}
