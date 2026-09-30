//! 🪚 `remove-member` — removes beam B1.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🪚remove-member/🪚beam-b1 — the committed vector.

/// 🪚 The committed `remove-member` vector holds the specification-vector law.
#[test]
fn remove_member_beam_b1() {
    super::assert_vector(super::Vector {
        kind: "remove-member",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🪚remove-member/🪚beam-b1/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🪚remove-member/🪚beam-b1/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🪚remove-member/🪚beam-b1/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🪚remove-member/🪚beam-b1/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🪚remove-member/🪚beam-b1/🎯️outcome/🔣️.json"),
    });
}
