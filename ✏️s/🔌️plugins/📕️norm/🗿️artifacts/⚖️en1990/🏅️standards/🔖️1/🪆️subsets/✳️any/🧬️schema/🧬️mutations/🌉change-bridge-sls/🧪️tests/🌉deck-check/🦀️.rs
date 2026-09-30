//! 🌉 `change-bridge-sls` — adds one deck serviceability record (acceleration, twist, deflection) for beam B1.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🌉change-bridge-sls/🌉deck-check — the committed vector.

/// 🌉 The committed `change-bridge-sls` vector holds the specification-vector law.
#[test]
fn change_bridge_sls_deck_check() {
    super::assert_vector(super::Vector {
        kind: "change-bridge-sls",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-bridge-sls/🌉deck-check/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-bridge-sls/🌉deck-check/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-bridge-sls/🌉deck-check/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-bridge-sls/🌉deck-check/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-bridge-sls/🌉deck-check/🎯️outcome/🔣️.json"),
    });
}
