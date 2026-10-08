//! ❌ `remove-seismic` — removes the seismic action E-1.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🕳️remove-seismic/✅apply — the committed vector.

fn vector() -> super::Vector {
    super::Vector {
        kind: "remove-seismic",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🕳️remove-seismic/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🕳️remove-seismic/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🕳️remove-seismic/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/🕳️remove-seismic/✅apply/🔺️diff/🔣️.json")),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🕳️remove-seismic/✅apply/🎯️outcome/🔣️.json"),
    }
}

/// ❌ The committed `remove-seismic` vector holds the specification-vector law.
#[test]
fn remove_seismic_e_1() {
    super::assert_vector(vector());
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_inputs(&vector());
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

