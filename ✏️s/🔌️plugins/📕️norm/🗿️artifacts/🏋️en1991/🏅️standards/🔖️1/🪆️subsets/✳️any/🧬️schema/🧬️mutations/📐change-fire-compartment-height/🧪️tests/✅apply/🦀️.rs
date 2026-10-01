//! 📐 `change-fire-compartment-height` — raises the fire compartment height from 3 m to 3.5 m.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/📐change-fire-compartment-height/✅apply — the committed vector.

/// 📐 The committed `change-fire-compartment-height` vector holds the specification-vector law.
#[test]
fn change_fire_compartment_height_3_5_m() {
    super::assert_vector(super::Vector {
        kind: "change-fire-compartment-height",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/📐change-fire-compartment-height/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/📐change-fire-compartment-height/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/📐change-fire-compartment-height/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/📐change-fire-compartment-height/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/📐change-fire-compartment-height/✅apply/🎯️outcome/🔣️.json"),
    });
}
