//! 📏 `insert-variable` — the canonical insert asked for a position past the list's end lands last under a `mutation.clamped` warning.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/📥insert-variable/📏clamp — the committed vector.

/// 📏 The committed `insert-variable` clamp vector holds the specification-vector law.
#[test]
fn insert_variable_clamp() {
    super::assert_vector(super::Vector {
        kind: "insert-variable",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/📥insert-variable/📏clamp/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/📥insert-variable/📏clamp/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/📥insert-variable/📏clamp/📸️snapshot/➡️after/🔣️.json"),
        diff: Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/📥insert-variable/📏clamp/🔺️diff/🔣️.json")),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/📥insert-variable/📏clamp/🎯️outcome/🔣️.json"),
    });
}
