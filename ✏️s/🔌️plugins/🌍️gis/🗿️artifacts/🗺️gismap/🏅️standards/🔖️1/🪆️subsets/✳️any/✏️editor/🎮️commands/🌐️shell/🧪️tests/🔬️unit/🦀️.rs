
use super::*;
use crate::editor::gis2d::Gis2dCommand;
use crate::editor::gis2d::testkit::{app, dispatch};

#[semio_framework_async_macros::async_test]
async fn open_source_on_an_unknown_feature_emits_no_effect() {
    let mut app = app().await;
    let result = dispatch(&mut app, Gis2dCommand::OpenSource(open_source::OpenSource { feature_id: "nope".into() })).await;
    assert!(result.mutations.is_empty());
    assert!(!result.requested_effects.iter().any(|effect| matches!(effect, Effect::OpenExternalUrl { .. })));
}

/// 🌐️ A Shell action never emits document operations — the registry's kind-discipline guard
/// rejects one that does.
#[semio_framework_async_macros::async_test]
async fn open_source_is_a_shell_action_that_emits_no_operations() {
    let definition = crate::editor::gis2d::create_gis2d_app();
    let action = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == "openSource").expect("openSource declared");
    assert!(matches!(action.kind, semio_framework_plugin::ActionKind::Shell));
    let mut app = crate::editor::gis2d::testkit::app_with_registry().await;
    assert!(dispatch(&mut app, Gis2dCommand::OpenSource(open_source::OpenSource { feature_id: "nope".into() })).await.mutations.is_empty());
}
