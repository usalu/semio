//! 🧲 `change-assumed-crane-horizontal` — assumes a 7.5 kN horizontal crane load.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/↔️change-assumed-crane-horizontal/✅apply — the committed vector.

/// 🧲 The committed `change-assumed-crane-horizontal` vector holds the specification-vector law.
#[test]
fn change_assumed_crane_horizontal_7_5_kn() {
    super::assert_vector(super::Vector {
        kind: "change-assumed-crane-horizontal",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/↔️change-assumed-crane-horizontal/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/↔️change-assumed-crane-horizontal/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/↔️change-assumed-crane-horizontal/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/↔️change-assumed-crane-horizontal/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/↔️change-assumed-crane-horizontal/✅apply/🎯️outcome/🔣️.json"),
    });
}
