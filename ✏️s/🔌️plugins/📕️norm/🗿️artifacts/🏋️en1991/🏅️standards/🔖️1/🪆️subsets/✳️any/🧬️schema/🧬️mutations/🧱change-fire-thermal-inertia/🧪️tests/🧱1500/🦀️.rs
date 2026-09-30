//! 🧱 `change-fire-thermal-inertia` — raises the enclosure thermal inertia b from 1160 to 1500 J/(m²s^½K).
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🧱change-fire-thermal-inertia/🧱1500 — the committed vector.

/// 🧱 The committed `change-fire-thermal-inertia` vector holds the specification-vector law.
#[test]
fn change_fire_thermal_inertia_1500() {
    super::assert_vector(super::Vector {
        kind: "change-fire-thermal-inertia",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱change-fire-thermal-inertia/🧱1500/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱change-fire-thermal-inertia/🧱1500/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱change-fire-thermal-inertia/🧱1500/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱change-fire-thermal-inertia/🧱1500/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱change-fire-thermal-inertia/🧱1500/🎯️outcome/🔣️.json"),
    });
}
