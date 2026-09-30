//! ➖ `remove-accidental-cases` — removes the vehicle impact case.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/➖️remove-accidental-cases/➖impact — the committed vector.

/// ➖ The committed `remove-accidental-cases` vector holds the specification-vector law.
#[test]
fn remove_accidental_cases_impact() {
    super::assert_vector(super::Vector {
        kind: "remove-accidental-cases",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-accidental-cases/➖impact/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-accidental-cases/➖impact/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-accidental-cases/➖impact/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-accidental-cases/➖impact/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-accidental-cases/➖impact/🎯️outcome/🔣️.json"),
    });
}
