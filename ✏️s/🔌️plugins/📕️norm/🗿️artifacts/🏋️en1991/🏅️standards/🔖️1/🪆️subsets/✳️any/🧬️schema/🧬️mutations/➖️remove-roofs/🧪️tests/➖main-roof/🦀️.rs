//! ➖ `remove-roofs` — removes the main duopitch roof.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/➖️remove-roofs/➖main-roof — the committed vector.

/// ➖ The committed `remove-roofs` vector holds the specification-vector law.
#[test]
fn remove_roofs_main_roof() {
    super::assert_vector(super::Vector {
        kind: "remove-roofs",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-roofs/➖main-roof/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-roofs/➖main-roof/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-roofs/➖main-roof/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-roofs/➖main-roof/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-roofs/➖main-roof/🎯️outcome/🔣️.json"),
    });
}
