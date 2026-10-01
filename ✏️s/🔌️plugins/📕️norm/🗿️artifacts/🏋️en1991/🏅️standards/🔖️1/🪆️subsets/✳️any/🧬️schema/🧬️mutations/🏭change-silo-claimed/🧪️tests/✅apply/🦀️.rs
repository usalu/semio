//! 🚫 `change-silo-claimed` — withdraws the silo claim.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🏭change-silo-claimed/✅apply — the committed vector.

/// 🚫 The committed `change-silo-claimed` vector holds the specification-vector law.
#[test]
fn change_silo_claimed_withdrawn() {
    super::assert_vector(super::Vector {
        kind: "change-silo-claimed",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏭change-silo-claimed/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏭change-silo-claimed/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏭change-silo-claimed/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏭change-silo-claimed/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏭change-silo-claimed/✅apply/🎯️outcome/🔣️.json"),
    });
}
