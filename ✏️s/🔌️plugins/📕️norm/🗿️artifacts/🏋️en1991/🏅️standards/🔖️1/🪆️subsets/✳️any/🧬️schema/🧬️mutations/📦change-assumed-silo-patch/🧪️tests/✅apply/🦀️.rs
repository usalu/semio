//! 📦 `change-assumed-silo-patch` — raises the assumed patch load from 1 kPa to 1.5 kPa.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/📦change-assumed-silo-patch/✅apply — the committed vector.

/// 📦 The committed `change-assumed-silo-patch` vector holds the specification-vector law.
#[test]
fn change_assumed_silo_patch_1_5_kpa() {
    super::assert_vector(super::Vector {
        kind: "change-assumed-silo-patch",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/📦change-assumed-silo-patch/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/📦change-assumed-silo-patch/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/📦change-assumed-silo-patch/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/📦change-assumed-silo-patch/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/📦change-assumed-silo-patch/✅apply/🎯️outcome/🔣️.json"),
    });
}
