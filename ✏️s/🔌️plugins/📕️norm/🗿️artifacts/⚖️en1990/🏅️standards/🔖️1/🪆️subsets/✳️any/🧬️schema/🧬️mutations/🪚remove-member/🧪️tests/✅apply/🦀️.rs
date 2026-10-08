//! 🪚 `remove-member` — removes beam B1.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🪚remove-member/✅apply — the committed vector.

fn vector() -> super::Vector {
    super::Vector {
        kind: "remove-member",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🪚remove-member/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🪚remove-member/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🪚remove-member/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/🪚remove-member/✅apply/🔺️diff/🔣️.json")),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🪚remove-member/✅apply/🎯️outcome/🔣️.json"),
    }
}

/// 🪚 The committed `remove-member` vector holds the specification-vector law.
#[test]
fn remove_member_beam_b1() {
    super::assert_vector(vector());
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    super::assert_inverse_sum_law(vector()).await;
}

