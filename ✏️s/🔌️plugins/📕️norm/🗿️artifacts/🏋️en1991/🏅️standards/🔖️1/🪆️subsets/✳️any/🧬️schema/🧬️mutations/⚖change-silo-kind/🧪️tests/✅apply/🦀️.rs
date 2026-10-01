//! 💧 `change-silo-kind` — reclassifies the container from a silo to a tank.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/⚖change-silo-kind/✅apply — the committed vector.

/// 💧 The committed `change-silo-kind` vector holds the specification-vector law.
#[test]
fn change_silo_kind_tank() {
    super::assert_vector(super::Vector {
        kind: "change-silo-kind",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/⚖change-silo-kind/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/⚖change-silo-kind/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/⚖change-silo-kind/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/⚖change-silo-kind/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/⚖change-silo-kind/✅apply/🎯️outcome/🔣️.json"),
    });
}
