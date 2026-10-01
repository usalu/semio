//! 🪝 `change-hoist-class` — upgrades the hoist from class HC2 to HC4.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/➕change-hoist-class/✅apply — the committed vector.

/// 🪝 The committed `change-hoist-class` vector holds the specification-vector law.
#[test]
fn change_hoist_class_hc4() {
    super::assert_vector(super::Vector {
        kind: "change-hoist-class",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕change-hoist-class/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕change-hoist-class/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕change-hoist-class/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕change-hoist-class/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕change-hoist-class/✅apply/🎯️outcome/🔣️.json"),
    });
}
