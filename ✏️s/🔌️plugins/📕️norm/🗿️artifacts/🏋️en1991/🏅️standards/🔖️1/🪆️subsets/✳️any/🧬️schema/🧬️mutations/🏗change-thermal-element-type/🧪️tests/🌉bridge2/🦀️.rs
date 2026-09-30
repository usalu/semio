//! 🌉 `change-thermal-element-type` — switches the thermal element from a building element to bridge deck type 2.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🏗change-thermal-element-type/🌉bridge2 — the committed vector.

/// 🌉 The committed `change-thermal-element-type` vector holds the specification-vector law.
#[test]
fn change_thermal_element_type_bridge2() {
    super::assert_vector(super::Vector {
        kind: "change-thermal-element-type",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏗change-thermal-element-type/🌉bridge2/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏗change-thermal-element-type/🌉bridge2/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏗change-thermal-element-type/🌉bridge2/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏗change-thermal-element-type/🌉bridge2/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏗change-thermal-element-type/🌉bridge2/🎯️outcome/🔣️.json"),
    });
}
