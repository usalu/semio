//! 💨 `change-air-density` — adopts the standard air density of 1.225 kg/m³.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🏢change-air-density/✅apply — the committed vector.

/// 💨 The committed `change-air-density` vector holds the specification-vector law.
#[test]
fn change_air_density_1_225() {
    super::assert_vector(super::Vector {
        kind: "change-air-density",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏢change-air-density/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏢change-air-density/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏢change-air-density/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏢change-air-density/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏢change-air-density/✅apply/🎯️outcome/🔣️.json"),
    });
}
