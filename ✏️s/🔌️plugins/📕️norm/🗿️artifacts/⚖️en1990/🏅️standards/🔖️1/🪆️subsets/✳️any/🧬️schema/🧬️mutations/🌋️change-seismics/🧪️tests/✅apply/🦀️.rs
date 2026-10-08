//! 🌋 `change-seismics` — raises the seismic importance class of E-1 from II to III.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🌋️change-seismics/✅apply — the committed vector.

fn vector() -> super::Vector {
    super::Vector {
        kind: "change-seismics",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌋️change-seismics/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌋️change-seismics/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌋️change-seismics/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/🌋️change-seismics/✅apply/🔺️diff/🔣️.json")),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌋️change-seismics/✅apply/🎯️outcome/🔣️.json"),
    }
}

/// 🌋 The committed `change-seismics` vector holds the specification-vector law.
#[test]
fn change_seismics_class_iii() {
    super::assert_vector(vector());
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_inputs(&vector());
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

