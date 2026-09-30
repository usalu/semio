//! ⛄ `change-snow-zone` — moves the site from snow zone 2 to snow zone 3.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🗺️change-snow-zone/⛄zone-3 — the committed vector.

/// ⛄ The committed `change-snow-zone` vector holds the specification-vector law.
#[test]
fn change_snow_zone_zone_3() {
    super::assert_vector(super::Vector {
        kind: "change-snow-zone",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🗺️change-snow-zone/⛄zone-3/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🗺️change-snow-zone/⛄zone-3/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🗺️change-snow-zone/⛄zone-3/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🗺️change-snow-zone/⛄zone-3/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🗺️change-snow-zone/⛄zone-3/🎯️outcome/🔣️.json"),
    });
}
