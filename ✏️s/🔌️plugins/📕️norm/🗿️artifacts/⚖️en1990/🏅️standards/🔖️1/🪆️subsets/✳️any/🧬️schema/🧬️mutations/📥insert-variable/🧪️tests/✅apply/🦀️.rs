//! 📥 `insert-variable` — adds a 15 kN snow action behind the wind action.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/📥insert-variable/✅apply — the committed vector.

fn vector() -> super::Vector {
    super::Vector {
        kind: "insert-variable",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/📥insert-variable/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/📥insert-variable/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/📥insert-variable/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/📥insert-variable/✅apply/🔺️diff/🔣️.json")),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/📥insert-variable/✅apply/🎯️outcome/🔣️.json"),
    }
}

/// 📥 The committed `insert-variable` vector holds the specification-vector law.
#[test]
fn insert_variable_snow() {
    super::assert_vector(vector());
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_inputs(&vector());
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

