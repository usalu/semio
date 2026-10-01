//! 🏬 `change-fire-occupancy` — reclassifies the fire occupancy from office to shopping.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🏢change-fire-occupancy/✅apply — the committed vector.

/// 🏬 The committed `change-fire-occupancy` vector holds the specification-vector law.
#[test]
fn change_fire_occupancy_shopping() {
    super::assert_vector(super::Vector {
        kind: "change-fire-occupancy",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏢change-fire-occupancy/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏢change-fire-occupancy/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏢change-fire-occupancy/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏢change-fire-occupancy/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏢change-fire-occupancy/✅apply/🎯️outcome/🔣️.json"),
    });
}
