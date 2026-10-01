//! 💥 `change-crane-class` — upgrades the crane from class HC2 to HC3.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/💥change-crane-class/✅apply — the committed vector.

/// 💥 The committed `change-crane-class` vector holds the specification-vector law.
#[test]
fn change_crane_class_hc3() {
    super::assert_vector(super::Vector {
        kind: "change-crane-class",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/💥change-crane-class/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/💥change-crane-class/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/💥change-crane-class/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/💥change-crane-class/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/💥change-crane-class/✅apply/🎯️outcome/🔣️.json"),
    });
}
