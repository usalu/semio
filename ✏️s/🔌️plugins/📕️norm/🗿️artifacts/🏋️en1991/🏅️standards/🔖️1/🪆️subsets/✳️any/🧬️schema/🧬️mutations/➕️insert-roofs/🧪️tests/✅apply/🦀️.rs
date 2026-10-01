//! ➕ `insert-roofs` — adds a monopitch annex roof with a parapet behind the main roof.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/➕️insert-roofs/✅apply — the committed vector.

/// ➕ The committed `insert-roofs` vector holds the specification-vector law.
#[test]
fn insert_roofs_annex_roof() {
    super::assert_vector(super::Vector {
        kind: "insert-roofs",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-roofs/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-roofs/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-roofs/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-roofs/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-roofs/✅apply/🎯️outcome/🔣️.json"),
    });
}
