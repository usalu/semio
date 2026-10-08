//! 🔍 `change-inspection-level` — tightens execution inspection from IL2 to IL3.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🔍change-inspection-level/✅apply — the committed vector.

fn vector() -> super::Vector {
    super::Vector {
        kind: "change-inspection-level",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔍change-inspection-level/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔍change-inspection-level/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔍change-inspection-level/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/🔍change-inspection-level/✅apply/🔺️diff/🔣️.json")),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔍change-inspection-level/✅apply/🎯️outcome/🔣️.json"),
    }
}

/// 🔍 The committed `change-inspection-level` vector holds the specification-vector law.
#[test]
fn change_inspection_level_il3() {
    super::assert_vector(vector());
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    super::assert_inverse_sum_law(vector()).await;
}

