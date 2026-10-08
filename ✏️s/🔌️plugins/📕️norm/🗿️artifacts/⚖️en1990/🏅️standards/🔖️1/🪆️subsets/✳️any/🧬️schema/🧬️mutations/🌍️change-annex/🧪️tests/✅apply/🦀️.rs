//! 🌍 `change-annex` — switches the national annex from the German NA to the recommended EN values.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🌍️change-annex/✅apply — the committed vector.

fn vector() -> super::Vector {
    super::Vector {
        kind: "change-annex",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌍️change-annex/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌍️change-annex/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌍️change-annex/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/🌍️change-annex/✅apply/🔺️diff/🔣️.json")),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌍️change-annex/✅apply/🎯️outcome/🔣️.json"),
    }
}

/// 🌍 The committed `change-annex` vector holds the specification-vector law.
#[test]
fn change_annex_en() {
    super::assert_vector(vector());
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    super::assert_inverse_sum_law(vector()).await;
}

