//! 🔍 `change-inspection-level` — tightens execution inspection from IL2 to IL3.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🔍change-inspection-level/🔍il3 — the committed vector.

/// 🔍 The committed `change-inspection-level` vector holds the specification-vector law.
#[test]
fn change_inspection_level_il3() {
    super::assert_vector(super::Vector {
        kind: "change-inspection-level",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔍change-inspection-level/🔍il3/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔍change-inspection-level/🔍il3/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔍change-inspection-level/🔍il3/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔍change-inspection-level/🔍il3/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔍change-inspection-level/🔍il3/🎯️outcome/🔣️.json"),
    });
}
