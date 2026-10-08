//! ⚓ `change-permanents` — raises the unfavourable permanent action G-sup from 80 kN to 95 kN.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/⚓️change-permanents/✅apply — the committed vector.

fn vector() -> super::Vector {
    super::Vector {
        kind: "change-permanents",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/⚓️change-permanents/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/⚓️change-permanents/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/⚓️change-permanents/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/⚓️change-permanents/✅apply/🔺️diff/🔣️.json")),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/⚓️change-permanents/✅apply/🎯️outcome/🔣️.json"),
    }
}

/// ⚓ The committed `change-permanents` vector holds the specification-vector law.
#[test]
fn change_permanents_95_kn() {
    super::assert_vector(vector());
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    super::assert_inverse_sum_law(vector()).await;
}

