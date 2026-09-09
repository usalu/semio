use super::*;

#[semio_framework_async_macros::async_test]
async fn the_mode_is_the_viewers_default() {
    assert_eq!(definition().id, crate::app_surface::MODE_VIEW);
}
