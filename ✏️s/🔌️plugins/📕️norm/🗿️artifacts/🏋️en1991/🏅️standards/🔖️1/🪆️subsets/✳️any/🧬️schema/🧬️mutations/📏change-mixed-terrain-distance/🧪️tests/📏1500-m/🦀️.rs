//! 📏 `change-mixed-terrain-distance` — moves the upwind terrain change from 0 m to 1500 m.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/📏change-mixed-terrain-distance/📏1500-m — the committed vector.

/// 📏 The committed `change-mixed-terrain-distance` vector holds the specification-vector law.
#[test]
fn change_mixed_terrain_distance_1500_m() {
    super::assert_vector(super::Vector {
        kind: "change-mixed-terrain-distance",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/📏change-mixed-terrain-distance/📏1500-m/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/📏change-mixed-terrain-distance/📏1500-m/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/📏change-mixed-terrain-distance/📏1500-m/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/📏change-mixed-terrain-distance/📏1500-m/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/📏change-mixed-terrain-distance/📏1500-m/🎯️outcome/🔣️.json"),
    });
}
