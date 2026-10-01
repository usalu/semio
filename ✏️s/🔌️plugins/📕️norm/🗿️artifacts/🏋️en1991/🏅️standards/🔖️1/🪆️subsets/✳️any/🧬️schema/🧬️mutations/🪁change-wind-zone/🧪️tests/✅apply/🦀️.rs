//! 🪁 `change-wind-zone` — moves the site from wind zone 2 to wind zone 3.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🪁change-wind-zone/✅apply — the committed vector.

/// 🪁 The committed `change-wind-zone` vector holds the specification-vector law.
#[test]
fn change_wind_zone_zone_3() {
    super::assert_vector(super::Vector {
        kind: "change-wind-zone",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🪁change-wind-zone/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🪁change-wind-zone/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🪁change-wind-zone/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🪁change-wind-zone/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🪁change-wind-zone/✅apply/🎯️outcome/🔣️.json"),
    });
}
