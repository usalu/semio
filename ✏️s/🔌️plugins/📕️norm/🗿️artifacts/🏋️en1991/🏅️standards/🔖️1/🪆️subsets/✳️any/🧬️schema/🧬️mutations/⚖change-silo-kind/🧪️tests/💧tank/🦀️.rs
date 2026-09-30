//! 💧 `change-silo-kind` — reclassifies the container from a silo to a tank.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/⚖change-silo-kind/💧tank — the committed vector.

/// 💧 The committed `change-silo-kind` vector holds the specification-vector law.
#[test]
fn change_silo_kind_tank() {
    super::assert_vector(super::Vector {
        kind: "change-silo-kind",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/⚖change-silo-kind/💧tank/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/⚖change-silo-kind/💧tank/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/⚖change-silo-kind/💧tank/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/⚖change-silo-kind/💧tank/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/⚖change-silo-kind/💧tank/🎯️outcome/🔣️.json"),
    });
}
