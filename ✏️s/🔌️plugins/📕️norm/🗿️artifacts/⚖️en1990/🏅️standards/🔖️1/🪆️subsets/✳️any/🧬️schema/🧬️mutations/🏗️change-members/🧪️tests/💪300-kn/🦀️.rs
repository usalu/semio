//! 💪 `change-members` — strengthens beam B1's STR resistance from 250 kN to 300 kN.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🏗️change-members/💪300-kn — the committed vector.

/// 💪 The committed `change-members` vector holds the specification-vector law.
#[test]
fn change_members_300_kn() {
    super::assert_vector(super::Vector {
        kind: "change-members",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏗️change-members/💪300-kn/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏗️change-members/💪300-kn/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏗️change-members/💪300-kn/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏗️change-members/💪300-kn/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏗️change-members/💪300-kn/🎯️outcome/🔣️.json"),
    });
}
