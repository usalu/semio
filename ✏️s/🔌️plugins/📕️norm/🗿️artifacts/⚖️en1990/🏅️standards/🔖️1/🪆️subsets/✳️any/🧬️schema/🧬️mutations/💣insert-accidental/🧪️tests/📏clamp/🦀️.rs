//! 📏 `insert-accidental` — the canonical insert asked for a position past the list's end lands last under a `mutation.clamped` warning.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/💣insert-accidental/📏clamp — the committed vector.

/// 📏 The committed `insert-accidental` clamp vector holds the specification-vector law.
#[test]
fn insert_accidental_clamp() {
    super::assert_vector(super::Vector {
        kind: "insert-accidental",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/💣insert-accidental/📏clamp/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/💣insert-accidental/📏clamp/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/💣insert-accidental/📏clamp/📸️snapshot/➡️after/🔣️.json"),
        diff: Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/💣insert-accidental/📏clamp/🔺️diff/🔣️.json")),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/💣insert-accidental/📏clamp/🎯️outcome/🔣️.json"),
    });
}
