//! 📥 `insert-variable` — adds a 15 kN snow action behind the wind action.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/📥insert-variable/✅apply — the committed vector.

/// 📥 The committed `insert-variable` vector holds the specification-vector law.
#[test]
fn insert_variable_snow() {
    super::assert_vector(super::Vector {
        kind: "insert-variable",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/📥insert-variable/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/📥insert-variable/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/📥insert-variable/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/📥insert-variable/✅apply/🔺️diff/🔣️.json")),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/📥insert-variable/✅apply/🎯️outcome/🔣️.json"),
    });
}
