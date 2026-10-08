//! 📎 `insert-effect` — adds a second office load path into beam B1 at half influence.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/📎insert-effect/✅apply — the committed vector.

fn vector() -> super::Vector {
    super::Vector {
        kind: "insert-effect",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/📎insert-effect/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/📎insert-effect/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/📎insert-effect/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/📎insert-effect/✅apply/🔺️diff/🔣️.json")),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/📎insert-effect/✅apply/🎯️outcome/🔣️.json"),
    }
}

/// 📎 The committed `insert-effect` vector holds the specification-vector law.
#[test]
fn insert_effect_office_half() {
    super::assert_vector(vector());
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_inputs(&vector());
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

