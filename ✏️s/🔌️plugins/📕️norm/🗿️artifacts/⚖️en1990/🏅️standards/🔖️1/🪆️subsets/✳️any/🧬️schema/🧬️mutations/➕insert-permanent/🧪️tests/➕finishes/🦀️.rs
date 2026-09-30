//! ➕ `insert-permanent` — adds a 12 kN unfavourable finishes action.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/➕insert-permanent/➕finishes — the committed vector.

/// ➕ The committed `insert-permanent` vector holds the specification-vector law.
#[test]
fn insert_permanent_finishes() {
    super::assert_vector(super::Vector {
        kind: "insert-permanent",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕insert-permanent/➕finishes/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕insert-permanent/➕finishes/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕insert-permanent/➕finishes/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕insert-permanent/➕finishes/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕insert-permanent/➕finishes/🎯️outcome/🔣️.json"),
    });
}
