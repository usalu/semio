//! 🎯 `change-reliability-class` — raises the reliability class from RC2 to RC3.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🎯change-reliability-class/✅apply — the committed vector.

fn vector() -> super::Vector {
    super::Vector {
        kind: "change-reliability-class",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🎯change-reliability-class/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🎯change-reliability-class/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🎯change-reliability-class/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/🎯change-reliability-class/✅apply/🔺️diff/🔣️.json")),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🎯change-reliability-class/✅apply/🎯️outcome/🔣️.json"),
    }
}

/// 🎯 The committed `change-reliability-class` vector holds the specification-vector law.
#[test]
fn change_reliability_class_rc3() {
    super::assert_vector(vector());
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    super::assert_inverse_sum_law(vector()).await;
}

