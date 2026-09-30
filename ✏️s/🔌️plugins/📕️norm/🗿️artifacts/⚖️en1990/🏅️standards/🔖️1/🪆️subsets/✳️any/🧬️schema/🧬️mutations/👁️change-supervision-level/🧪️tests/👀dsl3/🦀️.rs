//! 👀 `change-supervision-level` — tightens design supervision from DSL2 to DSL3.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/👁️change-supervision-level/👀dsl3 — the committed vector.

/// 👀 The committed `change-supervision-level` vector holds the specification-vector law.
#[test]
fn change_supervision_level_dsl3() {
    super::assert_vector(super::Vector {
        kind: "change-supervision-level",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/👁️change-supervision-level/👀dsl3/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/👁️change-supervision-level/👀dsl3/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/👁️change-supervision-level/👀dsl3/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/👁️change-supervision-level/👀dsl3/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/👁️change-supervision-level/👀dsl3/🎯️outcome/🔣️.json"),
    });
}
