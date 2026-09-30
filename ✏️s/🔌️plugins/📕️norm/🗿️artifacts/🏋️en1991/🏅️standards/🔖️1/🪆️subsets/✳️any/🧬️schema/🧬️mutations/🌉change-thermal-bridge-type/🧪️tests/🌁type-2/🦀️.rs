//! 🌁 `change-thermal-bridge-type` — switches the thermal bridge deck type from 1 to 2.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🌉change-thermal-bridge-type/🌁type-2 — the committed vector.

/// 🌁 The committed `change-thermal-bridge-type` vector holds the specification-vector law.
#[test]
fn change_thermal_bridge_type_type_2() {
    super::assert_vector(super::Vector {
        kind: "change-thermal-bridge-type",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-thermal-bridge-type/🌁type-2/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-thermal-bridge-type/🌁type-2/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-thermal-bridge-type/🌁type-2/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-thermal-bridge-type/🌁type-2/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-thermal-bridge-type/🌁type-2/🎯️outcome/🔣️.json"),
    });
}
