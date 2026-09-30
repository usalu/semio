//! ➖ `remove-floors` — removes the office floor.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/➖️remove-floors/➖office — the committed vector.

/// ➖ The committed `remove-floors` vector holds the specification-vector law.
#[test]
fn remove_floors_office() {
    super::assert_vector(super::Vector {
        kind: "remove-floors",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-floors/➖office/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-floors/➖office/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-floors/➖office/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-floors/➖office/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-floors/➖office/🎯️outcome/🔣️.json"),
    });
}
