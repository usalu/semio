//! 📤 `remove-variable` — removes the wind action Q-wind.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/📤remove-variable/📤wind — the committed vector.

/// 📤 The committed `remove-variable` vector holds the specification-vector law.
#[test]
fn remove_variable_wind() {
    super::assert_vector(super::Vector {
        kind: "remove-variable",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/📤remove-variable/📤wind/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/📤remove-variable/📤wind/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/📤remove-variable/📤wind/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/📤remove-variable/📤wind/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/📤remove-variable/📤wind/🎯️outcome/🔣️.json"),
    });
}
