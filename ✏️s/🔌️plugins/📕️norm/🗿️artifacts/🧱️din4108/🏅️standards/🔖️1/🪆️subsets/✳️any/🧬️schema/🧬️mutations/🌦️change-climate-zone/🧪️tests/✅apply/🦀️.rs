//! 🧫️ Canonical test of the committed `change-climate-zone` vector `✅apply` — the bundle is this implementation's own answer.

#[test]
fn committed_vector_holds() {
    super::assert_vector("🌦️change-climate-zone", "✅apply");
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    super::assert_inverse_sum_law("🌦️change-climate-zone", "✅apply").await;
}
