use super::*;
use semio_framework_plugin::{__semio_dispatch_PluginApp, plugin_app_close_prelude::*};

semio_framework_dispatch_macros::dyn_enum_close! {
    enum FlowViewerTestApps: semio_framework_plugin::PluginApp {
        FlowViewer(VcsArtifactApp<ViewerApp<FlowViewer>, semio_s_artifact_stdio_semio::SemioMembers>),
    }
}

#[semio_framework_async_macros::async_test]
async fn flow_viewer_member_factory_and_full_store_close_match_neutral_contract() {
    use semio_framework::kernel::{ArtifactKind, Rights, Scope};
    use semio_framework_plugin::{Plugin, PluginApp, PluginLifecycleStep};
    let fixture: Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧹️owners/🔣️.json")).unwrap();
    let definition = create_flow_viewer();
    assert_eq!(definition.role, AppRole::Viewer);
    assert_eq!(fixture["role"].as_str().unwrap(), "viewer");
    let id = definition.id.clone();
    let plugin = Plugin::<FlowViewerTestApps>::builder("flow-viewer-lifecycle")
        .label("Flow Viewer Lifecycle")
        .version("0.1.0")
        .package_id("semio:flow-viewer-lifecycle")
        .depends_on("flow", semio_framework::tree_pin!())
        .viewer::<FlowViewer>(definition)
        .try_build()
        .unwrap();
    let document_rights = plugin
        .manifest
        .capabilities
        .iter()
        .filter(|capability| matches!(capability.artifact, ArtifactKind::Document))
        .map(|capability| {
            assert!(matches!(capability.scope, Scope::App));
            if matches!(capability.rights, Rights::Read) {
                "read"
            } else {
                "unexpected"
            }
        })
        .collect::<Vec<_>>();
    assert_eq!(document_rights, fixture["documentRights"].as_array().unwrap().iter().map(|right| right.as_str().unwrap()).collect::<Vec<_>>());
    let mut app = plugin.create_app(&id, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).expect("registered Flow viewer factory must retain its typed member fleet");
    assert!(matches!(&app, FlowViewerTestApps::FlowViewer(_)));
    let items = fixture["grant"]["items"].as_u64().unwrap() as usize;
    let body = fixture["grant"]["bytes"].as_u64().unwrap() as usize;
    let mut completed = false;
    for _ in 0..fixture["maximumSteps"].as_u64().unwrap() {
        let demand = app.close_retirement_demands(body).expect("actual viewer quotes its next close turn");
        let grant = semio_framework_value::RetainedCloneGrant { maximum_items: items, maximum_copy_bytes: demand.copy_bytes.max(body), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
        match app.close_step(grant).expect("actual viewer closes through its declared five-lane owners") {
            PluginLifecycleStep::Progress(progress) => assert!(progress.fits(grant)),
            PluginLifecycleStep::Blocked { .. } => panic!("fresh viewer has no outstanding reader that may block close"),
            PluginLifecycleStep::AwaitingInput { reason } => panic!("fixture has no active worker input to await: {reason}"),
            PluginLifecycleStep::Complete(progress) => {
                assert!(progress.fits(grant));
                completed = true;
                break;
            }
        }
    }
    assert_eq!(completed, fixture["expected"]["complete"].as_bool().unwrap());
    assert_eq!(app.close_terminal_is_empty(), fixture["expected"]["terminalEmpty"].as_bool().unwrap());
}

#[semio_framework_async_macros::async_test]
async fn create_flow_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_flow_viewer();
    assert_eq!(def.role, AppRole::Viewer);
    assert_eq!(def.dialect, FLOW_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<FlowViewer as ArtifactViewer>::DIALECT, FLOW_DIALECT);
}
