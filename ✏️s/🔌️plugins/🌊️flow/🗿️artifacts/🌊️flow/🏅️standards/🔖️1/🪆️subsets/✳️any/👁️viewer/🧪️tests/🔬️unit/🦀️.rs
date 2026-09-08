
use super::*;

#[semio_framework_async_macros::async_test]
async fn flow_viewer_member_factory_and_full_store_close_match_neutral_contract() {
    use semio_framework::kernel::{ArtifactKind, Rights, Scope};
    use semio_framework_plugin::{Plugin, PluginApp, PluginCloseStep};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧹️owners/🔣️.json")).unwrap();
    let definition = create_flow_viewer();
    assert_eq!(definition.role, semio_framework_plugin::AppRole::Viewer);
    assert_eq!(fixture["role"].as_str().unwrap(), "viewer");
    let id = definition.id.clone();
    let plugin = Plugin::<crate::plugin::FlowApps>::builder("flow-viewer-lifecycle")
        .label("Flow Viewer Lifecycle")
        .version("0.1.0")
        .package_id("semio:flow-viewer-lifecycle")
        .viewer_with_members::<FlowViewer, semio_s_artifact_stdio_semio::SemioMembers>(definition)
        .try_build()
        .unwrap();
    let document_rights = plugin
        .manifest
        .capabilities
        .iter()
        .filter(|capability| matches!(capability.artifact, ArtifactKind::Document))
        .map(|capability| {
            assert!(matches!(capability.scope, Scope::App));
            if matches!(capability.rights, Rights::Read) { "read" } else { "unexpected" }
        })
        .collect::<Vec<_>>();
    assert_eq!(document_rights, fixture["documentRights"].as_array().unwrap().iter().map(|right| right.as_str().unwrap()).collect::<Vec<_>>());
    let mut app = plugin.create_app(&id).expect("registered Flow viewer factory must retain its typed member fleet");
    assert!(matches!(&app, crate::plugin::FlowApps::FlowViewer(_)));
    let items = fixture["grant"]["items"].as_u64().unwrap() as usize;
    let bytes = fixture["grant"]["bytes"].as_u64().unwrap() as usize;
    let mut completed = false;
    for _ in 0..fixture["maximumSteps"].as_u64().unwrap() {
        match app.close_step(items, bytes).expect("actual viewer closes through its declared five-lane owners") {
            PluginCloseStep::Pending { released_items, released_bytes } => assert!(released_items <= items && released_bytes <= bytes),
            PluginCloseStep::Blocked { .. } => panic!("fresh viewer has no outstanding reader that may block close"),
            PluginCloseStep::AwaitingInput { reason } => panic!("fixture has no active worker input to await: {reason}"),
            PluginCloseStep::Complete => {
                completed = true;
                break;
            }
        }
    }
    assert_eq!(completed, fixture["expected"]["complete"].as_bool().unwrap());
    assert_eq!(app.close_terminal_is_empty(), fixture["expected"]["terminalEmpty"].as_bool().unwrap());
    eprintln!("[DEBUG] real Flow viewer factory retained SemioMembers with read-only document rights and closed all five lanes plus framework interaction");
}

#[semio_framework_async_macros::async_test]
async fn create_flow_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_flow_viewer();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Viewer);
    assert_eq!(def.dialect, FLOW_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<FlowViewer as ArtifactViewer>::DIALECT, FLOW_DIALECT);
}
