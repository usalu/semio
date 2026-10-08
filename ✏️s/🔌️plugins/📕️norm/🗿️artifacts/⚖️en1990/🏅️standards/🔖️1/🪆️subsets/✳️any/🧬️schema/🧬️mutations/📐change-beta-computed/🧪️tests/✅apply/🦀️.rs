//! 📐 `change-beta-computed` — raises the computed reliability index β from 3.8 to 4.3.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/📐change-beta-computed/✅apply — the committed vector.

fn vector() -> super::Vector {
    super::Vector {
        kind: "change-beta-computed",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/📐change-beta-computed/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/📐change-beta-computed/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/📐change-beta-computed/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/📐change-beta-computed/✅apply/🔺️diff/🔣️.json")),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/📐change-beta-computed/✅apply/🎯️outcome/🔣️.json"),
    }
}

/// 📐 The committed `change-beta-computed` vector holds the specification-vector law.
#[test]
fn change_beta_computed_4_3() {
    super::assert_vector(vector());
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_inputs(&vector());
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

