
use super::*;
use crate::editor::layout::LayoutCommand;
use crate::editor::layout::testkit::{dispatch, layout_app};
use semio_framework::kernel::Effect;
use semio_framework_plugin::INTERACTION_SELECT_ACTION_ID;

#[semio_framework_async_macros::async_test]
async fn focus_preflight_issue_requests_a_select_effect_and_sets_active_page() {
    let mut app = layout_app().await;
    let result = dispatch(&mut app, LayoutCommand::FocusPreflightIssue(FocusPreflightIssue { object_id: Some("frame-1".into()), page_id: Some("page-2".into()) })).await;
    assert!(result.mutations.is_empty(), "preflight focus is config/effect-only, never a document operation");
    assert!(result.requested_effects.iter().any(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == INTERACTION_SELECT_ACTION_ID)), "must ask the host to redispatch interactionSelect");
}

#[semio_framework_async_macros::async_test]
async fn focus_preflight_issue_without_an_object_id_only_sets_active_page() {
    let mut app = layout_app().await;
    let result = dispatch(&mut app, LayoutCommand::FocusPreflightIssue(FocusPreflightIssue { object_id: None, page_id: Some("page-2".into()) })).await;
    assert!(result.requested_effects.is_empty(), "no object id means no select effect");
}
