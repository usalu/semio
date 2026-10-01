//! 🔩 `change-silo-k` — raises the lateral pressure ratio K from 0.4 to 0.55.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/⚙️change-silo-k/✅apply — the committed vector.

/// 🔩 The committed `change-silo-k` vector holds the specification-vector law.
#[test]
fn change_silo_k_0_55() {
    super::assert_vector(super::Vector {
        kind: "change-silo-k",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/⚙️change-silo-k/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/⚙️change-silo-k/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/⚙️change-silo-k/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/⚙️change-silo-k/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/⚙️change-silo-k/✅apply/🎯️outcome/🔣️.json"),
    });
}
