//! 🚫 `change-crane-claimed` — withdraws the crane claim.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🏗️change-crane-claimed/✅apply — the committed vector.

/// 🚫 The committed `change-crane-claimed` vector holds the specification-vector law.
#[test]
fn change_crane_claimed_withdrawn() {
    super::assert_vector(super::Vector {
        kind: "change-crane-claimed",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏗️change-crane-claimed/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏗️change-crane-claimed/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏗️change-crane-claimed/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏗️change-crane-claimed/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏗️change-crane-claimed/✅apply/🎯️outcome/🔣️.json"),
    });
}
