#[allow(unused_imports)] use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
#[allow(unused_imports)] use semio_framework_value_derive::{FromValue, ToValue};
#[allow(unused_imports)] use semio_s_artifact_layout_layout_editor_commands::delete_selection::*;
use semio_s_artifact_layout_layout_editor_panels::focus_preflight_issue::FocusPreflightIssue;
use semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::blueprint::config::LayoutBlueprintWindowConfigOwner;
use semio_s_artifact_layout_layout_editor::unit_tests::context::layout_app_with_registry;
use semio_s_artifact_layout_layout_editor::LayoutCommand;
use semio_s_artifact_layout_layout_editor_commands::LAYOUT_INTERACTION_ELEMENTS;
use semio_framework_plugin::artifact_app_laws;
use semio_framework_plugin::{ActionMeta, PluginApp, ViewModel, ViewWindowInstance, WindowConfigOwner};

async fn selected_frames(app: &semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<semio_s_artifact_layout_layout_editor::LayoutPlayApp>>) -> Vec<String> {
    app.interaction_state().await.selection.get(LAYOUT_INTERACTION_ELEMENTS).map(|selection| selection.ids.clone()).unwrap_or_default()
}

/// ⚖️ LAW: `deleteSelection` removes every frame in the live `"elements"` selection.
#[semio_framework_async_macros::async_test]
async fn delete_selection_removes_the_live_selected_frame() {
    let mut app = layout_app_with_registry().await;
    let frame_id = app.snapshot().expect("projection").pages[0].frames.first().expect("frame").id().to_string();
    let view = ViewModel { window_instances: vec![ViewWindowInstance { id: "layout-blueprint".into(), window_kind_id: LayoutBlueprintWindowConfigOwner::WINDOW_KIND_ID.into() }], ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    let meta = ActionMeta { view_state: Some(view.for_window_instance("layout-blueprint").expect("blueprint window instance")), ..artifact_app_laws::meta("local") };
    app.bind_instance_id(meta.instance_id).await;
    app.dispatch_typed(LayoutCommand::FocusPreflightIssue(FocusPreflightIssue { object_id: Some(frame_id.clone()), page_id: Some("page-1".into()) }), &meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("select frame via preflight focus");
    artifact_app_laws::settle_registered_typed_operation(&mut app.0, meta.instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("focus settles");
    assert_eq!(selected_frames(&app).await, vec![frame_id.clone()]);
    app.dispatch_typed(LayoutCommand::DeleteSelection(DeleteSelection {}), &meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("deleteSelection");
    artifact_app_laws::settle_registered_typed_operation(&mut app.0, meta.instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("delete settles");
    let snapshot = app.snapshot().expect("projection");
    assert!(!snapshot.pages[0].frames.iter().any(|frame| frame.id() == frame_id), "deleteSelection must remove the selected frame");
    artifact_app_laws::close_registered_fixture_app(&mut app.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT);
}
