//! 🧪️ `change-concentrated-bearing-area` — the committed applied vector's inverse diffs sum to the negative of its forward diff.

#[semio_framework_async_macros::async_test]
async fn change_concentrated_bearing_area_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = crate::mutations::fixture_tests::applied_vector("change-concentrated-bearing-area");
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}
