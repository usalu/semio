//! ➖ `remove-permanent` — removes the favourable permanent action G-inf.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/➖remove-permanent/✅apply — the committed vector.

fn vector() -> super::Vector {
    super::Vector {
        kind: "remove-permanent",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖remove-permanent/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖remove-permanent/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖remove-permanent/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/➖remove-permanent/✅apply/🔺️diff/🔣️.json")),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖remove-permanent/✅apply/🎯️outcome/🔣️.json"),
    }
}

/// ➖ The committed `remove-permanent` vector holds the specification-vector law.
#[test]
fn remove_permanent_g_inf() {
    super::assert_vector(vector());
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_inputs(&vector());
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

