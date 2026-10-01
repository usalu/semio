//! 🧭 `change-mixed-terrain-upwind` — smooths the upwind terrain of a mixed profile from category II to category I.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🧭change-mixed-terrain-upwind/✅apply — the committed vector.

/// 🧭 The committed `change-mixed-terrain-upwind` vector holds the specification-vector law.
#[test]
fn change_mixed_terrain_upwind_class_1() {
    super::assert_vector(super::Vector {
        kind: "change-mixed-terrain-upwind",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭change-mixed-terrain-upwind/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭change-mixed-terrain-upwind/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭change-mixed-terrain-upwind/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭change-mixed-terrain-upwind/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭change-mixed-terrain-upwind/✅apply/🎯️outcome/🔣️.json"),
    });
}
