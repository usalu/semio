//! 🏢 `change-storey-count` — raises the storey count from 3 to 5.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🏙change-storey-count/✅apply — the committed vector.

/// 🏢 The committed `change-storey-count` vector holds the specification-vector law.
#[test]
fn change_storey_count_5_storeys() {
    super::assert_vector(super::Vector {
        kind: "change-storey-count",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏙change-storey-count/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏙change-storey-count/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏙change-storey-count/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏙change-storey-count/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏙change-storey-count/✅apply/🎯️outcome/🔣️.json"),
    });
}
