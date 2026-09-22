use super::*;
use crate::editor::layout::unit_tests::context::{dispatch, layout_app, layout_app_with_registry};
use crate::editor::layout::modes::edit::windows::blueprint::config::LayoutBlueprintWindowConfigOwner;
use crate::editor::layout::{LayoutCommand, LAYOUT_INTERACTION_ELEMENTS};
use semio_framework::kernel::Effect;
use semio_framework_plugin::{artifact_app_laws, ActionMeta, PluginApp, ViewModel, ViewWindowInstance, WindowConfigOwner};

/// 🕹️ Ticket 26/09/16/INPUT-CAUSALITY-LEDGER §2 C: the `interactionSelect` this command emits as an
/// `Effect::DispatchAction` is folded in-reactor by the typed-operation ladder, so the witness is
/// the selection state once the retained publication settles. That needs everything the plugin
/// host supplies and the bare `dispatch` helper does not: the manifest registry (the verb is a
/// `Migrated` row there), a bound live instance, a `ViewModel` naming the Blueprint window the
/// command addresses (the retained work's `extent` refuses a command without one), and the host's
/// settle protocol (`settle_registered_typed_operation`) — exactly `🧪️tests/🔬️window-ownership`'s
/// recipe. Fails-before: the effect rode the receipt's `effects` to the host and selection landed
/// one guest round trip later.
#[semio_framework_async_macros::async_test]
async fn focus_preflight_issue_selects_the_object_inline_and_sets_active_page() {
    let mut app = layout_app_with_registry().await;
    let view = ViewModel { window_instances: vec![ViewWindowInstance { id: "layout-blueprint".into(), window_kind_id: LayoutBlueprintWindowConfigOwner::WINDOW_KIND_ID.into() }], ..Default::default() };
    let meta = ActionMeta { view_state: Some(view.for_window_instance("layout-blueprint").expect("blueprint window instance")), ..artifact_app_laws::meta("local") };
    app.bind_instance_id(meta.instance_id).await;
    app.dispatch_typed(LayoutCommand::FocusPreflightIssue(FocusPreflightIssue { object_id: Some("frame-1".into()), page_id: Some("page-2".into()) }), &meta).await.expect("dispatch");
    let receipt = artifact_app_laws::settle_registered_typed_operation(&mut app.0, meta.instance_id).await.expect("retained publication settles");
    assert!(!receipt.effects.iter().any(|effect| matches!(effect, Effect::DispatchAction { .. })), "interactionSelect is folded in-reactor, never handed to the host: {:?}", receipt.effects);
    let selected = app.interaction_state().await.selection.get(LAYOUT_INTERACTION_ELEMENTS).map(|selection| selection.ids.clone()).unwrap_or_default();
    assert_eq!(selected, vec!["frame-1".to_string()], "the focused object is selected inside the carrying operation");
    artifact_app_laws::close_registered_fixture_app(&mut app.0);
}

#[semio_framework_async_macros::async_test]
async fn focus_preflight_issue_without_an_object_id_only_sets_active_page() {
    let mut app = layout_app().await;
    let result = dispatch(&mut app, LayoutCommand::FocusPreflightIssue(FocusPreflightIssue { object_id: None, page_id: Some("page-2".into()) })).await;
    assert!(result.requested_effects.is_empty(), "no object id means no select effect");
}
