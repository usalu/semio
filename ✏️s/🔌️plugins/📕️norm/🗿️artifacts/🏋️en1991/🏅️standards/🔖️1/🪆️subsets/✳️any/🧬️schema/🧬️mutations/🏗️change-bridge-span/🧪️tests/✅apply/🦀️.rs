//! 🌉 `change-bridge-span` — lengthens the bridge span from 20 m to 36 m.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🏗️change-bridge-span/✅apply — the committed vector.

/// 🌉 The committed `change-bridge-span` vector holds the specification-vector law.
#[test]
fn change_bridge_span_36_m() {
    super::assert_vector(super::Vector {
        kind: "change-bridge-span",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏗️change-bridge-span/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏗️change-bridge-span/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏗️change-bridge-span/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏗️change-bridge-span/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏗️change-bridge-span/✅apply/🎯️outcome/🔣️.json"),
    });
}
