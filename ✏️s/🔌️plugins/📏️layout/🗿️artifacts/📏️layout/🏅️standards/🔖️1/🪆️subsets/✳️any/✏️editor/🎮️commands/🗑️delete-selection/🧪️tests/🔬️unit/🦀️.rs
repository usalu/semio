use super::*;
use crate::editor::layout::commands::focus_preflight_issue::FocusPreflightIssue;
use crate::editor::layout::modes::edit::windows::blueprint::config::LayoutBlueprintWindowConfigOwner;
use crate::editor::layout::unit_tests::context::layout_app_with_registry;
use crate::editor::layout::{LayoutCommand, LAYOUT_INTERACTION_ELEMENTS};
use semio_framework_plugin::artifact_app_laws;
use semio_framework_plugin::{ActionMeta, PluginApp, ViewModel, ViewWindowInstance, WindowConfigOwner};

async fn selected_frames(app: &semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::layout::LayoutPlayApp>>) -> Vec<String> {
    app.interaction_state().await.selection.get(LAYOUT_INTERACTION_ELEMENTS).map(|selection| selection.ids.clone()).unwrap_or_default()
}

/// ⚖️ LAW: `deleteSelection` removes every frame in the live `"elements"` selection.
#[semio_framework_async_macros::async_test]
async fn delete_selection_removes_the_live_selected_frame() {
    let mut app = layout_app_with_registry().await;
    let frame_id = app.snapshot().expect("projection").pages[0].frames.first().expect("frame").id().to_string();
    let view = ViewModel { window_instances: vec![ViewWindowInstance { id: "layout-blueprint".into(), window_kind_id: LayoutBlueprintWindowConfigOwner::WINDOW_KIND_ID.into() }], ..Default::default() };
    let meta = ActionMeta { view_state: Some(view.for_window_instance("layout-blueprint").expect("blueprint window instance")), ..artifact_app_laws::meta("local") };
    app.bind_instance_id(meta.instance_id).await;
    app.dispatch_typed(LayoutCommand::FocusPreflightIssue(FocusPreflightIssue { object_id: Some(frame_id.clone()), page_id: Some("page-1".into()) }), &meta).await.expect("select frame via preflight focus");
    artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id).await.expect("focus settles");
    assert_eq!(selected_frames(&app).await, vec![frame_id.clone()]);
    app.dispatch_typed(LayoutCommand::DeleteSelection(DeleteSelection {}), &meta).await.expect("deleteSelection");
    artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id).await.expect("delete settles");
    let snapshot = app.snapshot().expect("projection");
    assert!(!snapshot.pages[0].frames.iter().any(|frame| frame.id() == frame_id), "deleteSelection must remove the selected frame");
    artifact_app_laws::close_registered_fixture_app(&mut app);
}
