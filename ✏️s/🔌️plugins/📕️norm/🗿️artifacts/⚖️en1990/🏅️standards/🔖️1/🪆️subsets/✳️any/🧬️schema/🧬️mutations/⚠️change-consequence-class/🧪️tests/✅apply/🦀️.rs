//! 🚨 `change-consequence-class` — escalates the building from consequence class CC2 to CC3.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/⚠️change-consequence-class/✅apply — the committed vector.

fn vector() -> super::Vector {
    super::Vector {
        kind: "change-consequence-class",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/⚠️change-consequence-class/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/⚠️change-consequence-class/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/⚠️change-consequence-class/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/⚠️change-consequence-class/✅apply/🔺️diff/🔣️.json")),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/⚠️change-consequence-class/✅apply/🎯️outcome/🔣️.json"),
    }
}

/// 🚨 The committed `change-consequence-class` vector holds the specification-vector law.
#[test]
fn change_consequence_class_cc3() {
    super::assert_vector(vector());
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_inputs(&vector());
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

