use crate::editor::flow::FlowPlayApp;
use crate::viewer::flow::FlowViewer;

#[semio_framework_async_macros::async_test]
async fn flow_actual_surface_factories_close_all_owners_under_neutral_grants() {
    use semio_framework_plugin::{AppRole, PluginApp, PluginCloseStep};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧹️surface-owners/🔣️.json")).unwrap();
    let plugin = super::plugin().expect("the actual Flow package must assemble every registered surface");
    assert_eq!(plugin.manifest.apps.len(), fixture["expected"]["factories"].as_u64().unwrap() as usize);
    let roles = plugin
        .manifest
        .apps
        .iter()
        .map(|definition| match definition.role {
            AppRole::Editor => "editor",
            AppRole::Viewer => "viewer",
        })
        .collect::<Vec<_>>();
    assert_eq!(roles, fixture["roles"].as_array().unwrap().iter().map(|role| role.as_str().unwrap()).collect::<Vec<_>>());
    let items = fixture["items"].as_u64().unwrap() as usize;
    for definition in &plugin.manifest.apps {
        for grant in fixture["byteGrants"].as_array().unwrap() {
            let bytes = grant.as_u64().unwrap() as usize;
            let mut app = plugin.create_app(&definition.id).expect("every declared surface has an actual factory");
            assert!(matches!((&app, &definition.role), (super::FlowApps::FlowEditor(_), AppRole::Editor) | (super::FlowApps::FlowViewer(_), AppRole::Viewer)));
            let mut completed = false;
            for _ in 0..fixture["maximumSteps"].as_u64().unwrap() {
                match app.close_step(items, bytes).expect("the actual Flow app owns every store and instance close stage") {
                    PluginCloseStep::Pending { released_items, released_bytes } => assert!(released_items <= items && released_bytes <= bytes),
                    PluginCloseStep::Blocked { reason } => panic!("fresh Flow surface retains no external reader: {reason}"),
                    PluginCloseStep::AwaitingInput { reason } => panic!("fixture has no active worker input to await: {reason}"),
                    PluginCloseStep::Complete => {
                        completed = true;
                        break;
                    }
                }
            }
            assert_eq!(completed, fixture["expected"]["complete"].as_bool().unwrap(), "{} bytes={bytes}", definition.id);
            assert_eq!(app.close_terminal_is_empty(), fixture["expected"]["terminalEmpty"].as_bool().unwrap());
            assert!(matches!(app.close_step(0, 0).unwrap(), PluginCloseStep::Pending { released_items: 0, released_bytes: 0 }));
            assert!(app.close_terminal_is_empty());
            assert!(matches!(app.close_step(items, bytes).unwrap(), PluginCloseStep::Complete));
            eprintln!("[DEBUG] actual Flow surface={} bytes={} closed all stores and app-instance owners", definition.id, bytes);
        }
    }
}

/// 👁️ A viewer instance never mutates the document store, even when dispatched.
#[semio_framework_async_macros::async_test]
async fn flow_viewer_never_mutates() {
    semio_framework_plugin::artifact_app_laws::assert_viewer_never_mutates::<FlowViewer>().await;
}

/// 🤝️ Editor and viewer surfaces agree on the artifact dialect they address.
#[semio_framework_async_macros::async_test]
async fn flow_editor_and_viewer_share_dialect() {
    semio_framework_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<FlowPlayApp, FlowViewer>().await;
}
