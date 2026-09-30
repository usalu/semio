//! 📥 `insert-variable` — adds a 15 kN snow action behind the wind action.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/📥insert-variable/📥snow — the committed vector.

/// 📥 The committed `insert-variable` vector holds the specification-vector law.
#[test]
fn insert_variable_snow() {
    super::assert_vector(super::Vector {
        kind: "insert-variable",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/📥insert-variable/📥snow/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/📥insert-variable/📥snow/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/📥insert-variable/📥snow/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/📥insert-variable/📥snow/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/📥insert-variable/📥snow/🎯️outcome/🔣️.json"),
    });
}
