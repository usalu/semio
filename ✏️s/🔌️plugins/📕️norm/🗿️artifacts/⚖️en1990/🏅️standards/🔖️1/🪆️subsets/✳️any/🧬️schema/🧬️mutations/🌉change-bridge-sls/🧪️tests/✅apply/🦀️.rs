//! 🌉 `change-bridge-sls` — adds one deck serviceability record (acceleration, twist, deflection) for beam B1.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🌉change-bridge-sls/✅apply — the committed vector.

/// 🌉 The committed `change-bridge-sls` vector holds the specification-vector law.
#[test]
fn change_bridge_sls_deck_check() {
    super::assert_vector(super::Vector {
        kind: "change-bridge-sls",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-bridge-sls/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-bridge-sls/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-bridge-sls/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-bridge-sls/✅apply/🔺️diff/🔣️.json")),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-bridge-sls/✅apply/🎯️outcome/🔣️.json"),
    });
}
