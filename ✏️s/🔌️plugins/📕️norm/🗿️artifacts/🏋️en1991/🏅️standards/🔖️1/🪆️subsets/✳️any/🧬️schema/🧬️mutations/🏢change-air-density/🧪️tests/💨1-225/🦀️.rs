//! 💨 `change-air-density` — adopts the standard air density of 1.225 kg/m³.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🏢change-air-density/💨1-225 — the committed vector.

/// 💨 The committed `change-air-density` vector holds the specification-vector law.
#[test]
fn change_air_density_1_225() {
    super::assert_vector(super::Vector {
        kind: "change-air-density",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏢change-air-density/💨1-225/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏢change-air-density/💨1-225/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏢change-air-density/💨1-225/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏢change-air-density/💨1-225/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏢change-air-density/💨1-225/🎯️outcome/🔣️.json"),
    });
}
