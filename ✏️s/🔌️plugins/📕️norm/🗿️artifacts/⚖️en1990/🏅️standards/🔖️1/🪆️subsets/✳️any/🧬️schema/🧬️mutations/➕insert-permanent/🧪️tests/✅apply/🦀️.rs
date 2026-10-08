//! ➕ `insert-permanent` — adds a 12 kN unfavourable finishes action.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/➕insert-permanent/✅apply — the committed vector.

fn vector() -> super::Vector {
    super::Vector {
        kind: "insert-permanent",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕insert-permanent/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕insert-permanent/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕insert-permanent/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/➕insert-permanent/✅apply/🔺️diff/🔣️.json")),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕insert-permanent/✅apply/🎯️outcome/🔣️.json"),
    }
}

/// ➕ The committed `insert-permanent` vector holds the specification-vector law.
#[test]
fn insert_permanent_finishes() {
    super::assert_vector(vector());
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    super::assert_inverse_sum_law(vector()).await;
}

