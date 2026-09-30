//! 🧭 `change-mixed-terrain-upwind` — smooths the upwind terrain of a mixed profile from category II to category I.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🧭change-mixed-terrain-upwind/🧭class-1 — the committed vector.

/// 🧭 The committed `change-mixed-terrain-upwind` vector holds the specification-vector law.
#[test]
fn change_mixed_terrain_upwind_class_1() {
    super::assert_vector(super::Vector {
        kind: "change-mixed-terrain-upwind",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭change-mixed-terrain-upwind/🧭class-1/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭change-mixed-terrain-upwind/🧭class-1/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭change-mixed-terrain-upwind/🧭class-1/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭change-mixed-terrain-upwind/🧭class-1/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭change-mixed-terrain-upwind/🧭class-1/🎯️outcome/🔣️.json"),
    });
}
