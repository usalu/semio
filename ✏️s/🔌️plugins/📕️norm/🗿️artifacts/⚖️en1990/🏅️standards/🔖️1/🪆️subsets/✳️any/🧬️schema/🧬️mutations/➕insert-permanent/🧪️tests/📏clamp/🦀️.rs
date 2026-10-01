//! 📏 `insert-permanent` — the canonical insert asked for a position past the list's end lands last under a `mutation.clamped` warning.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/➕insert-permanent/📏clamp — the committed vector.

/// 📏 The committed `insert-permanent` clamp vector holds the specification-vector law.
#[test]
fn insert_permanent_clamp() {
    super::assert_vector(super::Vector {
        kind: "insert-permanent",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕insert-permanent/📏clamp/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕insert-permanent/📏clamp/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕insert-permanent/📏clamp/📸️snapshot/➡️after/🔣️.json"),
        diff: Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/➕insert-permanent/📏clamp/🔺️diff/🔣️.json")),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕insert-permanent/📏clamp/🎯️outcome/🔣️.json"),
    });
}
