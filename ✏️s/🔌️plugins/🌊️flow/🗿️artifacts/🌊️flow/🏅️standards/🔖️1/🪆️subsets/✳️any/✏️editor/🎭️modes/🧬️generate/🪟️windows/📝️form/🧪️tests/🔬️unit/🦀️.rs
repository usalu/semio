
use super::*;
use crate::editor::flow::testkit::{flow_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn without_a_generation_the_form_shows_the_placeholder_copy() {
    let mut app = flow_app().await;
    assert!(render_body(&mut app, FLOW_PLAY_BODY_GENERATE_FORM).await.contains("Add a generation"));
}
