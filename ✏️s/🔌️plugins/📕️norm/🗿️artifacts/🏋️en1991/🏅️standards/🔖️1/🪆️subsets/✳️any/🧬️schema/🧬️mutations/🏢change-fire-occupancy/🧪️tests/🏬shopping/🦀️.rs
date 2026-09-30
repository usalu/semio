//! 🏬 `change-fire-occupancy` — reclassifies the fire occupancy from office to shopping.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🏢change-fire-occupancy/🏬shopping — the committed vector.

/// 🏬 The committed `change-fire-occupancy` vector holds the specification-vector law.
#[test]
fn change_fire_occupancy_shopping() {
    super::assert_vector(super::Vector {
        kind: "change-fire-occupancy",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏢change-fire-occupancy/🏬shopping/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏢change-fire-occupancy/🏬shopping/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏢change-fire-occupancy/🏬shopping/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏢change-fire-occupancy/🏬shopping/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏢change-fire-occupancy/🏬shopping/🎯️outcome/🔣️.json"),
    });
}
