//! 🧫️ Canonical test of the committed `change-airtightness-n50` vector `✅apply` — the bundle is this implementation's own answer.

#[test]
fn committed_vector_holds() {
    super::assert_vector("💨️change-airtightness-n50", "✅apply");
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    super::assert_inverse_sum_law("💨️change-airtightness-n50", "✅apply").await;
}
