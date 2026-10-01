//! ➖ `remove-floors` — removes the office floor.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/➖️remove-floors/✅apply — the committed vector.

/// ➖ The committed `remove-floors` vector holds the specification-vector law.
#[test]
fn remove_floors_office() {
    super::assert_vector(super::Vector {
        kind: "remove-floors",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-floors/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-floors/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-floors/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-floors/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-floors/✅apply/🎯️outcome/🔣️.json"),
    });
}
