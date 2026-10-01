//! 🌳 `change-terrain-category` — roughens the terrain from category II to category III.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🏞️change-terrain-category/✅apply — the committed vector.

/// 🌳 The committed `change-terrain-category` vector holds the specification-vector law.
#[test]
fn change_terrain_category_class_3() {
    super::assert_vector(super::Vector {
        kind: "change-terrain-category",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏞️change-terrain-category/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏞️change-terrain-category/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏞️change-terrain-category/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏞️change-terrain-category/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏞️change-terrain-category/✅apply/🎯️outcome/🔣️.json"),
    });
}
