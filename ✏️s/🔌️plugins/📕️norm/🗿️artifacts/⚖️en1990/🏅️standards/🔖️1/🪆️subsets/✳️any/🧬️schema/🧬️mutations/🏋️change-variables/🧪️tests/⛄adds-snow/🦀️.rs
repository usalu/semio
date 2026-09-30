//! ⛄ `change-variables` — replaces the variable actions with a set that adds a 15 kN snow action.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🏋️change-variables/⛄adds-snow — the committed vector.

/// ⛄ The committed `change-variables` vector holds the specification-vector law.
#[test]
fn change_variables_adds_snow() {
    super::assert_vector(super::Vector {
        kind: "change-variables",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏋️change-variables/⛄adds-snow/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏋️change-variables/⛄adds-snow/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏋️change-variables/⛄adds-snow/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏋️change-variables/⛄adds-snow/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏋️change-variables/⛄adds-snow/🎯️outcome/🔣️.json"),
    });
}
