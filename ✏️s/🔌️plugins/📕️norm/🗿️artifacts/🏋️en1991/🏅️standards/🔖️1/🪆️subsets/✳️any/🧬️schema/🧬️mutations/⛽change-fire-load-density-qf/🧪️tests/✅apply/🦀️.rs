//! ⛽ `change-fire-load-density-qf` — raises the characteristic fire load density q_f,k from 420 MJ/m² to 600 MJ/m².
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/⛽change-fire-load-density-qf/✅apply — the committed vector.

/// ⛽ The committed `change-fire-load-density-qf` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-fire-load-density-qf",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/⛽change-fire-load-density-qf/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/⛽change-fire-load-density-qf/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/⛽change-fire-load-density-qf/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/⛽change-fire-load-density-qf/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/⛽change-fire-load-density-qf/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_fire_load_density_qf_600_mj() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-fire-load-density-qf` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_fire_load_density_qf_600_mj_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}
