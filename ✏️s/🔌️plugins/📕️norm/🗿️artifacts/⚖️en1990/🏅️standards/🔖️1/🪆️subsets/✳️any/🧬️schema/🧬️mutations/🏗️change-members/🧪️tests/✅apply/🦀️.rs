//! 💪 `change-members` — strengthens beam B1's STR resistance from 250 kN to 300 kN.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🏗️change-members/✅apply — the committed vector.

/// 💪 The committed `change-members` vector holds the specification-vector law.
#[test]
fn change_members_300_kn() {
    super::assert_vector(super::Vector {
        kind: "change-members",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏗️change-members/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏗️change-members/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏗️change-members/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/🏗️change-members/✅apply/🔺️diff/🔣️.json")),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏗️change-members/✅apply/🎯️outcome/🔣️.json"),
    });
}
