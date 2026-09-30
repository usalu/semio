//! 🎯 `change-reliability-class` — raises the reliability class from RC2 to RC3.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🎯change-reliability-class/🎯rc3 — the committed vector.

/// 🎯 The committed `change-reliability-class` vector holds the specification-vector law.
#[test]
fn change_reliability_class_rc3() {
    super::assert_vector(super::Vector {
        kind: "change-reliability-class",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🎯change-reliability-class/🎯rc3/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🎯change-reliability-class/🎯rc3/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🎯change-reliability-class/🎯rc3/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🎯change-reliability-class/🎯rc3/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🎯change-reliability-class/🎯rc3/🎯️outcome/🔣️.json"),
    });
}
