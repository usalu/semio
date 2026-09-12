use super::*;
use crate::editor::sequence::unit_tests::context::{new_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn catalogue_lists_step_kind_actions() {
    let mut app = new_app().await;
    assert!(render_body(&mut app, SEQUENCE_PLAY_BODY_CATALOGUE).await.contains("sequence-play-catalogue.action.log.print"));
}
