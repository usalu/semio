//! 📤 `remove-variable` — removes the wind action Q-wind.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/📤remove-variable/✅apply — the committed vector.

/// 📤 The committed `remove-variable` vector holds the specification-vector law.
#[test]
fn remove_variable_wind() {
    super::assert_vector(super::Vector {
        kind: "remove-variable",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/📤remove-variable/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/📤remove-variable/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/📤remove-variable/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/📤remove-variable/✅apply/🔺️diff/🔣️.json")),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/📤remove-variable/✅apply/🎯️outcome/🔣️.json"),
    });
}
