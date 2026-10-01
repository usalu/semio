//! 💣 `insert-accidental` — adds a 30 kN explosion action.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/💣insert-accidental/✅apply — the committed vector.

/// 💣 The committed `insert-accidental` vector holds the specification-vector law.
#[test]
fn insert_accidental_explosion() {
    super::assert_vector(super::Vector {
        kind: "insert-accidental",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/💣insert-accidental/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/💣insert-accidental/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/💣insert-accidental/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/💣insert-accidental/✅apply/🔺️diff/🔣️.json")),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/💣insert-accidental/✅apply/🎯️outcome/🔣️.json"),
    });
}
