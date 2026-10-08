//! 🌋 `insert-seismic` — adds a 45 kN seismic action E-2 of importance class IV.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🌋insert-seismic/✅apply — the committed vector.

fn vector() -> super::Vector {
    super::Vector {
        kind: "insert-seismic",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌋insert-seismic/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌋insert-seismic/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌋insert-seismic/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/🌋insert-seismic/✅apply/🔺️diff/🔣️.json")),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌋insert-seismic/✅apply/🎯️outcome/🔣️.json"),
    }
}

/// 🌋 The committed `insert-seismic` vector holds the specification-vector law.
#[test]
fn insert_seismic_class_iv() {
    super::assert_vector(vector());
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_inputs(&vector());
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

