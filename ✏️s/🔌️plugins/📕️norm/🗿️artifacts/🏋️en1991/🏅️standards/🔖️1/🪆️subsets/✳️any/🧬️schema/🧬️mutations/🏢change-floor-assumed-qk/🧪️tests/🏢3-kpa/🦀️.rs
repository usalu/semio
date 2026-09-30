//! 🏢 `change-floor-assumed-qk` — raises the assumed imposed load q_k of floor 0 from 1 kPa to 3 kPa.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🏢change-floor-assumed-qk/🏢3-kpa — the committed vector.

/// 🏢 The committed `change-floor-assumed-qk` vector holds the specification-vector law.
#[test]
fn change_floor_assumed_qk_3_kpa() {
    super::assert_vector(super::Vector {
        kind: "change-floor-assumed-qk",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏢change-floor-assumed-qk/🏢3-kpa/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏢change-floor-assumed-qk/🏢3-kpa/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏢change-floor-assumed-qk/🏢3-kpa/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏢change-floor-assumed-qk/🏢3-kpa/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏢change-floor-assumed-qk/🏢3-kpa/🎯️outcome/🔣️.json"),
    });
}
