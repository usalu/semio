//! 🌉 `change-structure-kind` — reclassifies the structure from a building to a bridge.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🌉change-structure-kind/✅apply — the committed vector.

/// 🌉 The committed `change-structure-kind` vector holds the specification-vector law.
#[test]
fn change_structure_kind_bridge() {
    super::assert_vector(super::Vector {
        kind: "change-structure-kind",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-structure-kind/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-structure-kind/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-structure-kind/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-structure-kind/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-structure-kind/✅apply/🎯️outcome/🔣️.json"),
    });
}
