//! 📏 `insert-member` — the canonical insert asked for a position past the list's end lands last under a `mutation.clamped` warning.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🔩insert-member/📏clamp — the committed vector.

/// 📏 The committed `insert-member` clamp vector holds the specification-vector law.
#[test]
fn insert_member_clamp() {
    super::assert_vector(super::Vector {
        kind: "insert-member",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔩insert-member/📏clamp/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔩insert-member/📏clamp/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔩insert-member/📏clamp/📸️snapshot/➡️after/🔣️.json"),
        diff: Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/🔩insert-member/📏clamp/🔺️diff/🔣️.json")),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔩insert-member/📏clamp/🎯️outcome/🔣️.json"),
    });
}
