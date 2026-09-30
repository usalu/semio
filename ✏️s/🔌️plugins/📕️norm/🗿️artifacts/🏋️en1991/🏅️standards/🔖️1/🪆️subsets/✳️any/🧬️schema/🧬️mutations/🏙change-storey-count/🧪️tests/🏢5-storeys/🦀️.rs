//! 🏢 `change-storey-count` — raises the storey count from 3 to 5.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🏙change-storey-count/🏢5-storeys — the committed vector.

/// 🏢 The committed `change-storey-count` vector holds the specification-vector law.
#[test]
fn change_storey_count_5_storeys() {
    super::assert_vector(super::Vector {
        kind: "change-storey-count",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏙change-storey-count/🏢5-storeys/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏙change-storey-count/🏢5-storeys/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏙change-storey-count/🏢5-storeys/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏙change-storey-count/🏢5-storeys/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏙change-storey-count/🏢5-storeys/🎯️outcome/🔣️.json"),
    });
}
