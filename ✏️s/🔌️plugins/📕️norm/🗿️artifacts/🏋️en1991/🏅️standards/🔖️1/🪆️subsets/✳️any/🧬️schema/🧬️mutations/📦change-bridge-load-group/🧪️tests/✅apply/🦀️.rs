//! 📦 `change-bridge-load-group` — switches the traffic load group from gr1a to gr1b.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/📦change-bridge-load-group/✅apply — the committed vector.

/// 📦 The committed `change-bridge-load-group` vector holds the specification-vector law.
#[test]
fn change_bridge_load_group_gr1b() {
    super::assert_vector(super::Vector {
        kind: "change-bridge-load-group",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/📦change-bridge-load-group/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/📦change-bridge-load-group/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/📦change-bridge-load-group/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/📦change-bridge-load-group/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/📦change-bridge-load-group/✅apply/🎯️outcome/🔣️.json"),
    });
}
