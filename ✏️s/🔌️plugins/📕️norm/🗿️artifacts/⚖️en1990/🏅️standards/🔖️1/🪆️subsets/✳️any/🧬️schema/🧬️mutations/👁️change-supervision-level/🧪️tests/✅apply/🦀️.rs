//! 👀 `change-supervision-level` — tightens design supervision from DSL2 to DSL3.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/👁️change-supervision-level/✅apply — the committed vector.

fn vector() -> super::Vector {
    super::Vector {
        kind: "change-supervision-level",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/👁️change-supervision-level/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/👁️change-supervision-level/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/👁️change-supervision-level/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/👁️change-supervision-level/✅apply/🔺️diff/🔣️.json")),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/👁️change-supervision-level/✅apply/🎯️outcome/🔣️.json"),
    }
}

/// 👀 The committed `change-supervision-level` vector holds the specification-vector law.
#[test]
fn change_supervision_level_dsl3() {
    super::assert_vector(vector());
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_inputs(&vector());
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

