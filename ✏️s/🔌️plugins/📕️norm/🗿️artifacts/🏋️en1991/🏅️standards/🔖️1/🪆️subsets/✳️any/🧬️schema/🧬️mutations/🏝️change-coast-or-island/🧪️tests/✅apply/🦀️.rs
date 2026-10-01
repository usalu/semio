//! 🌊 `change-coast-or-island` — places the site on the coast.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🏝️change-coast-or-island/✅apply — the committed vector.

/// 🌊 The committed `change-coast-or-island` vector holds the specification-vector law.
#[test]
fn change_coast_or_island_coast() {
    super::assert_vector(super::Vector {
        kind: "change-coast-or-island",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏝️change-coast-or-island/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏝️change-coast-or-island/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏝️change-coast-or-island/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏝️change-coast-or-island/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏝️change-coast-or-island/✅apply/🎯️outcome/🔣️.json"),
    });
}
