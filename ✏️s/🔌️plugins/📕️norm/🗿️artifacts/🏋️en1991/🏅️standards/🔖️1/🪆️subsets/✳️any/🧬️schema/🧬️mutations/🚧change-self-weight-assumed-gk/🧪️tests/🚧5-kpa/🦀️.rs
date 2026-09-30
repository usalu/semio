//! 🚧 `change-self-weight-assumed-gk` — raises the assumed self-weight g_k of element 0 from 2 kPa to 5 kPa.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🚧change-self-weight-assumed-gk/🚧5-kpa — the committed vector.

/// 🚧 The committed `change-self-weight-assumed-gk` vector holds the specification-vector law.
#[test]
fn change_self_weight_assumed_gk_5_kpa() {
    super::assert_vector(super::Vector {
        kind: "change-self-weight-assumed-gk",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚧change-self-weight-assumed-gk/🚧5-kpa/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚧change-self-weight-assumed-gk/🚧5-kpa/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚧change-self-weight-assumed-gk/🚧5-kpa/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚧change-self-weight-assumed-gk/🚧5-kpa/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚧change-self-weight-assumed-gk/🚧5-kpa/🎯️outcome/🔣️.json"),
    });
}
