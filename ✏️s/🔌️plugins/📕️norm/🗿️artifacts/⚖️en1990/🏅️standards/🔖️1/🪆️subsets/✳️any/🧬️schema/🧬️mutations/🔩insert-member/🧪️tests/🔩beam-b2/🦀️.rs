//! 🔩 `insert-member` — adds a 7.5 m office beam B2 behind beam B1.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🔩insert-member/🔩beam-b2 — the committed vector.

/// 🔩 The committed `insert-member` vector holds the specification-vector law.
#[test]
fn insert_member_beam_b2() {
    super::assert_vector(super::Vector {
        kind: "insert-member",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔩insert-member/🔩beam-b2/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔩insert-member/🔩beam-b2/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔩insert-member/🔩beam-b2/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔩insert-member/🔩beam-b2/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔩insert-member/🔩beam-b2/🎯️outcome/🔣️.json"),
    });
}
