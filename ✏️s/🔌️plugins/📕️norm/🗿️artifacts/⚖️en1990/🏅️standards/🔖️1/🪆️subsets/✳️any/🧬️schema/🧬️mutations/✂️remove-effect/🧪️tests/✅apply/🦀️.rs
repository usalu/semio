//! 🔌 `remove-effect` — detaches the seismic action E-1 from beam B1.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/✂️remove-effect/✅apply — the committed vector.

fn vector() -> super::Vector {
    super::Vector {
        kind: "remove-effect",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/✂️remove-effect/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/✂️remove-effect/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/✂️remove-effect/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/✂️remove-effect/✅apply/🔺️diff/🔣️.json")),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/✂️remove-effect/✅apply/🎯️outcome/🔣️.json"),
    }
}

/// 🔌 The committed `remove-effect` vector holds the specification-vector law.
#[test]
fn remove_effect_e_1() {
    super::assert_vector(vector());
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_inputs(&vector());
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

