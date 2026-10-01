//! 📏 `change-mixed-terrain-distance` — moves the upwind terrain change from 0 m to 1500 m.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/📏change-mixed-terrain-distance/✅apply — the committed vector.

/// 📏 The committed `change-mixed-terrain-distance` vector holds the specification-vector law.
#[test]
fn change_mixed_terrain_distance_1500_m() {
    super::assert_vector(super::Vector {
        kind: "change-mixed-terrain-distance",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/📏change-mixed-terrain-distance/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/📏change-mixed-terrain-distance/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/📏change-mixed-terrain-distance/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/📏change-mixed-terrain-distance/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/📏change-mixed-terrain-distance/✅apply/🎯️outcome/🔣️.json"),
    });
}
