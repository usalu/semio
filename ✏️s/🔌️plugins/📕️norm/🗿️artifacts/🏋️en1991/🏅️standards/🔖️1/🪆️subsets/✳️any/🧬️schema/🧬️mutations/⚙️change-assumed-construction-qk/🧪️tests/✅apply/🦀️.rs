//! 👷 `change-assumed-construction-qk` — raises the assumed construction load q_ca from 1500 Pa to 2000 Pa.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/⚙️change-assumed-construction-qk/✅apply — the committed vector.

/// 👷 The committed `change-assumed-construction-qk` vector holds the specification-vector law.
#[test]
fn change_assumed_construction_qk_2_kpa() {
    super::assert_vector(super::Vector {
        kind: "change-assumed-construction-qk",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/⚙️change-assumed-construction-qk/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/⚙️change-assumed-construction-qk/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/⚙️change-assumed-construction-qk/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/⚙️change-assumed-construction-qk/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/⚙️change-assumed-construction-qk/✅apply/🎯️outcome/🔣️.json"),
    });
}
