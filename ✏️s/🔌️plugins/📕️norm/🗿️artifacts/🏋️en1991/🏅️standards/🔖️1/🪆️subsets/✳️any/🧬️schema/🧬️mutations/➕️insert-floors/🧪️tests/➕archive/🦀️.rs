//! ➕ `insert-floors` — adds an archive floor (category E1) behind the office floor.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/➕️insert-floors/➕archive — the committed vector.

/// ➕ The committed `insert-floors` vector holds the specification-vector law.
#[test]
fn insert_floors_archive() {
    super::assert_vector(super::Vector {
        kind: "insert-floors",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-floors/➕archive/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-floors/➕archive/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-floors/➕archive/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-floors/➕archive/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-floors/➕archive/🎯️outcome/🔣️.json"),
    });
}
