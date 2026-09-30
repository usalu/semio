//! ➕ `insert-accidental-cases` — adds an internal explosion case behind the vehicle impact case.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/➕️insert-accidental-cases/➕explosion — the committed vector.

/// ➕ The committed `insert-accidental-cases` vector holds the specification-vector law.
#[test]
fn insert_accidental_cases_explosion() {
    super::assert_vector(super::Vector {
        kind: "insert-accidental-cases",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-accidental-cases/➕explosion/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-accidental-cases/➕explosion/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-accidental-cases/➕explosion/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-accidental-cases/➕explosion/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-accidental-cases/➕explosion/🎯️outcome/🔣️.json"),
    });
}
