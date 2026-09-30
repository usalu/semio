//! ➕ `insert-roofs` — adds a monopitch annex roof with a parapet behind the main roof.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/➕️insert-roofs/➕annex-roof — the committed vector.

/// ➕ The committed `insert-roofs` vector holds the specification-vector law.
#[test]
fn insert_roofs_annex_roof() {
    super::assert_vector(super::Vector {
        kind: "insert-roofs",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-roofs/➕annex-roof/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-roofs/➕annex-roof/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-roofs/➕annex-roof/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-roofs/➕annex-roof/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-roofs/➕annex-roof/🎯️outcome/🔣️.json"),
    });
}
