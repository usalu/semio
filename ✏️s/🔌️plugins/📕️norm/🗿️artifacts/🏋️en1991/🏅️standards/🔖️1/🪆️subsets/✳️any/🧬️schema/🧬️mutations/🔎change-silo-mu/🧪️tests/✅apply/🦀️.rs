//! 🔎 `change-silo-mu` — raises the wall friction coefficient μ from 0.4 to 0.5.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🔎change-silo-mu/✅apply — the committed vector.

/// 🔎 The committed `change-silo-mu` vector holds the specification-vector law.
#[test]
fn change_silo_mu_0_5() {
    super::assert_vector(super::Vector {
        kind: "change-silo-mu",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔎change-silo-mu/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔎change-silo-mu/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔎change-silo-mu/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔎change-silo-mu/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔎change-silo-mu/✅apply/🎯️outcome/🔣️.json"),
    });
}
