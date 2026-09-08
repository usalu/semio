
use super::*;
use crate::editor::generation3d::testkit::{app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn generate_form_hints_without_a_selected_generation() {
    let mut app = app().await;
    assert!(render_body(&mut app, GENERATION_3D_PLAY_BODY_GENERATE_FORM).await.contains("Add a generation"));
}
