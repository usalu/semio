//! 🌳 `change-terrain-category` — roughens the terrain from category II to category III.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🏞️change-terrain-category/🌳class-3 — the committed vector.

/// 🌳 The committed `change-terrain-category` vector holds the specification-vector law.
#[test]
fn change_terrain_category_class_3() {
    super::assert_vector(super::Vector {
        kind: "change-terrain-category",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏞️change-terrain-category/🌳class-3/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏞️change-terrain-category/🌳class-3/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏞️change-terrain-category/🌳class-3/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏞️change-terrain-category/🌳class-3/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏞️change-terrain-category/🌳class-3/🎯️outcome/🔣️.json"),
    });
}
