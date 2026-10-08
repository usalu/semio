//! 🧫️ Canonical test of the committed `remove-zone-window` vector `✅apply` — the bundle is this implementation's own answer.

#[test]
fn committed_vector_holds() {
    super::assert_vector("🚫️remove-zone-window", "✅apply");
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    super::assert_inverse_sum_law("🚫️remove-zone-window", "✅apply").await;
}
