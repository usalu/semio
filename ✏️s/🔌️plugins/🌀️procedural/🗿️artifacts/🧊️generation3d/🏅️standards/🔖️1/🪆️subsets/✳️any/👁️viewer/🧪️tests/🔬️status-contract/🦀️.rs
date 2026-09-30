use super::*;

const PREVIEW_CANCEL_FIXTURE_JSON: &str = include_str!("../../../🧫️fixtures/🛑️preview-cancel.json");
const VIEWER_SURFACE_ID: &str = "viewer";

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct PreviewCancelFixture { format: String, version: u8, status_contract: StatusContract }
#[derive(serde::Deserialize)]
struct StatusContract { surfaces: Vec<SurfaceRow> }
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct SurfaceRow { surface: String, cancel_action: String, framework_reserved_verb: bool }


fn preview_cancel_fixture() -> PreviewCancelFixture {
    let fixture: PreviewCancelFixture = serde_json::from_str(PREVIEW_CANCEL_FIXTURE_JSON).expect("preview cancel fixture");
    assert_eq!(fixture.format, "semio.generation3d.preview-cancel");
    assert_eq!(fixture.version, 1);
    fixture
}

#[test]
fn the_viewer_publishes_the_framework_run_abort_it_earns_by_declaring_the_run() {
    let fixture = preview_cancel_fixture();
    let viewer = fixture.status_contract.surfaces.iter().find(|row| row.surface == VIEWER_SURFACE_ID).expect("viewer surface row");
    assert!(viewer.framework_reserved_verb, "the fixture states the published verb is framework-reserved");
    assert_eq!(viewer.cancel_action, semio_framework_tool_run::TOOL_RUN_ABORT_ACTION_ID);
    let definition = create_generation3d_viewer();
    assert!(definition.tools.iter().any(|tool| tool.id == preview_eval::PREVIEW_EVAL_TOOL_ID && tool.run.is_some()), "the viewer declares the preview-evaluation RUN, which is what injects the abort");
    assert!(definition.modes.iter().any(|mode| mode.tools.iter().any(|tool_ref| tool_ref.as_str() == preview_eval::PREVIEW_EVAL_TOOL_ID)), "a declared tool no mode references is refused outright by the builder");
    assert!(!definition.commands.iter().any(|command| command.id == viewer.cancel_action), "the abort is injected, never a plugin command");
    // 🧯️ The kernel RELEASE the abort's close hands the extensions is the plugin's own retained verb,
    // and it is the one that still needs a route and a host-only lane.
    assert!(GENERATION3D_VIEW_FLOW_EVAL_TOOL_IDS.contains(&"flowEvalRelease"), "the release hop needs a retained route");
    let release = definition.commands.iter().find(|command| command.id == "flowEvalRelease").expect("the viewer declares the release hop");
    assert_eq!(release.kind, ActionKind::View, "a release retires ephemeral runtime work, never the document");
    assert_eq!(release.semantics.execution.interactive_job, InteractiveJobClassification::Migrated, "an unclassified command is rejected with interactive-job.not-ui-safe");
    let contract = <Generation3dViewFlowEvalJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS
        .iter()
        .find(|contract| contract.tool_id == "flowEvalRelease")
        .expect("the release hop needs an exact publication contract");
    assert_eq!(contract.lanes, &[ArtifactToolPublicationLane::HostOnly], "a release publishes no store lane at all");
}
