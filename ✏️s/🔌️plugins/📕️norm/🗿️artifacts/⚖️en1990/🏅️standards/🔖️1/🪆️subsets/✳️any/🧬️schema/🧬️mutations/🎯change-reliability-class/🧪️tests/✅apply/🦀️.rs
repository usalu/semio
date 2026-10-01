//! 🎯 `change-reliability-class` — raises the reliability class from RC2 to RC3.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🎯change-reliability-class/✅apply — the committed vector.

/// 🎯 The committed `change-reliability-class` vector holds the specification-vector law.
#[test]
fn change_reliability_class_rc3() {
    super::assert_vector(super::Vector {
        kind: "change-reliability-class",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🎯change-reliability-class/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🎯change-reliability-class/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🎯change-reliability-class/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/🎯change-reliability-class/✅apply/🔺️diff/🔣️.json")),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🎯change-reliability-class/✅apply/🎯️outcome/🔣️.json"),
    });
}
