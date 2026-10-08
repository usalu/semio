//! 🔗 `change-effects` — halves the influence of the wind action on beam B1.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🔗change-effects/✅apply — the committed vector.

fn vector() -> super::Vector {
    super::Vector {
        kind: "change-effects",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔗change-effects/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔗change-effects/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔗change-effects/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/🔗change-effects/✅apply/🔺️diff/🔣️.json")),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔗change-effects/✅apply/🎯️outcome/🔣️.json"),
    }
}

/// 🔗 The committed `change-effects` vector holds the specification-vector law.
#[test]
fn change_effects_half_wind() {
    super::assert_vector(vector());
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_inputs(&vector());
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

