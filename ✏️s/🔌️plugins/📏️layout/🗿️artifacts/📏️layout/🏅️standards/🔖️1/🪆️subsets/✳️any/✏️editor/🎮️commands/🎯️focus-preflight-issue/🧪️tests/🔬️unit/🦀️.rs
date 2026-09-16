use super::*;
use crate::editor::layout::unit_tests::context::{dispatch, layout_app, layout_app_with_registry};
use crate::editor::layout::{LayoutCommand, LAYOUT_INTERACTION_ELEMENTS};
use semio_framework::kernel::Effect;
use semio_framework_plugin::{artifact_app_laws, PluginApp};

/// 🕹️ Ticket 26/09/16/INPUT-CAUSALITY-LEDGER §2 C: the `interactionSelect` this command emits as an
/// `Effect::DispatchAction` is folded in-reactor, so the witness is the selection state — which
/// needs the manifest registry (the verb is a `Migrated` row there) and a bound live instance.
#[semio_framework_async_macros::async_test]
async fn focus_preflight_issue_selects_the_object_inline_and_sets_active_page() {
    let mut app = layout_app_with_registry().await;
    app.bind_instance_id(artifact_app_laws::meta("local").instance_id).await;
    let result = dispatch(&mut app, LayoutCommand::FocusPreflightIssue(FocusPreflightIssue { object_id: Some("frame-1".into()), page_id: Some("page-2".into()) })).await;
    assert!(result.mutations.is_empty(), "preflight focus is config/effect-only, never a document operation");
    assert!(!result.requested_effects.iter().any(|effect| matches!(effect, Effect::DispatchAction { .. })), "interactionSelect is folded in-reactor, never handed to the host: {:?}", result.requested_effects);
    let selected = app.interaction_state().await.selection.get(LAYOUT_INTERACTION_ELEMENTS).map(|selection| selection.ids.clone()).unwrap_or_default();
    assert_eq!(selected, vec!["frame-1".to_string()], "the focused object is selected inside the carrying turn (diagnostics: {:?}, output: {:?})", result.diagnostics, result.output);
    artifact_app_laws::close_registered_fixture_app(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn focus_preflight_issue_without_an_object_id_only_sets_active_page() {
    let mut app = layout_app().await;
    let result = dispatch(&mut app, LayoutCommand::FocusPreflightIssue(FocusPreflightIssue { object_id: None, page_id: Some("page-2".into()) })).await;
    assert!(result.requested_effects.is_empty(), "no object id means no select effect");
}
