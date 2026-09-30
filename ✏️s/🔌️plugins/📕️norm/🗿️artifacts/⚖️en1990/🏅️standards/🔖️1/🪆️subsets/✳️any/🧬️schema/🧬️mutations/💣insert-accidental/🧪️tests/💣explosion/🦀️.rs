//! 💣 `insert-accidental` — adds a 30 kN explosion action.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/💣insert-accidental/💣explosion — the committed vector.

/// 💣 The committed `insert-accidental` vector holds the specification-vector law.
#[test]
fn insert_accidental_explosion() {
    super::assert_vector(super::Vector {
        kind: "insert-accidental",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/💣insert-accidental/💣explosion/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/💣insert-accidental/💣explosion/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/💣insert-accidental/💣explosion/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/💣insert-accidental/💣explosion/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/💣insert-accidental/💣explosion/🎯️outcome/🔣️.json"),
    });
}
