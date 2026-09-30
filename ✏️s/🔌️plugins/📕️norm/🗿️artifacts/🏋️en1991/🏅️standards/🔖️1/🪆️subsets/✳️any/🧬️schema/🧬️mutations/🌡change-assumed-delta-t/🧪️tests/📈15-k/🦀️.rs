//! 📈 `change-assumed-delta-t` — raises the assumed uniform temperature difference from 10 K to 15 K.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🌡change-assumed-delta-t/📈15-k — the committed vector.

/// 📈 The committed `change-assumed-delta-t` vector holds the specification-vector law.
#[test]
fn change_assumed_delta_t_15_k() {
    super::assert_vector(super::Vector {
        kind: "change-assumed-delta-t",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌡change-assumed-delta-t/📈15-k/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌡change-assumed-delta-t/📈15-k/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌡change-assumed-delta-t/📈15-k/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌡change-assumed-delta-t/📈15-k/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌡change-assumed-delta-t/📈15-k/🎯️outcome/🔣️.json"),
    });
}
