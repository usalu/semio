//! 🔎 `change-silo-mu` — raises the wall friction coefficient μ from 0.4 to 0.5.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🔎change-silo-mu/🔎0-5 — the committed vector.

/// 🔎 The committed `change-silo-mu` vector holds the specification-vector law.
#[test]
fn change_silo_mu_0_5() {
    super::assert_vector(super::Vector {
        kind: "change-silo-mu",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔎change-silo-mu/🔎0-5/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔎change-silo-mu/🔎0-5/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔎change-silo-mu/🔎0-5/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔎change-silo-mu/🔎0-5/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔎change-silo-mu/🔎0-5/🎯️outcome/🔣️.json"),
    });
}
