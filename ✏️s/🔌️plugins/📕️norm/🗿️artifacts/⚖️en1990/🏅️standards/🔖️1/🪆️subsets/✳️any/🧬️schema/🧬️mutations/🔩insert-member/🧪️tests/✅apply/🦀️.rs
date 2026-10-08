//! 🔩 `insert-member` — adds a 7.5 m office beam B2 behind beam B1.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🔩insert-member/✅apply — the committed vector.

fn vector() -> super::Vector {
    super::Vector {
        kind: "insert-member",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔩insert-member/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔩insert-member/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔩insert-member/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/🔩insert-member/✅apply/🔺️diff/🔣️.json")),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔩insert-member/✅apply/🎯️outcome/🔣️.json"),
    }
}

/// 🔩 The committed `insert-member` vector holds the specification-vector law.
#[test]
fn insert_member_beam_b2() {
    super::assert_vector(vector());
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    super::assert_inverse_sum_law(vector()).await;
}

