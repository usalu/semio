use super::*;
use crate::editor::flow::unit_tests::context::{composed_scene, dispatch, flow_app, settle};
use crate::editor::flow::FlowCommand;
use crate::examples::demo;

/// ⚖️ LAW: `setActiveExample demo` turns the live document into exactly the published demo graph —
/// widgets, synapses and layout — not merely into a different widget count (an empty demo satisfied
/// that, and painted an empty canvas in play).
#[semio_framework_async_macros::async_test]
async fn set_active_example_demo_loads_the_published_demo_graph() {
    let mut app = flow_app().await;
    let mut expected = demo::snapshot_from_text(demo::PRIMARY_TEXT).expect("demo parses").to_host_snapshot();
    expected.schema = crate::FLOW_DOCUMENT_SCHEMA.into();
    let before = composed_scene(&app).await;
    assert_ne!(before, expected, "the live default document must differ from the demo so a demo load is observable");
    before.retire_cold();
    dispatch(&mut app, FlowCommand::SetActiveExample(SetActiveExample { example_id: demo::ID.into() })).await;
    settle(&mut app).await;
    let after = composed_scene(&app).await;
    assert_eq!(after, expected, "setActiveExample demo must load the published demo graph");
    after.retire_cold();
    expected.retire_cold();
}

/// 📚️ LAW: every example the editor ships loads through its own `setActiveExample` route — the reducer accepts each
/// published id on the genesis document, and its asset parses through the same boundary the route reads.
#[test]
fn every_shipped_example_loads_through_set_active_example() {
    use semio_framework_plugin::ArtifactEditor;
    let examples = crate::editor::flow::FlowPlayApp::examples();
    assert!(!examples.is_empty(), "flow ships at least one example");
    for example in &examples {
        let loaded = demo::snapshot_from_text(&example.document_json()).unwrap_or_else(|error| panic!("example {:?} must parse through the route's own boundary: {error}", example.id()));
        loaded.to_host_snapshot().retire_cold();
        let composed = FlowSnapshot::default();
        let mut emit = set_active_example_edit(&SetActiveExample { example_id: example.id().to_string() }, &composed).unwrap_or_else(|fault| panic!("setActiveExample {:?} must load its own shipped asset: {}", example.id(), fault.message));
        let mut ready = false;
        for _ in 0..4096 {
            match emit.prepare_child_one(1, 65_536).expect("bounded child preparation") {
                semio_framework_plugin::app::ChildEmitPreparationStep::Ready => {
                    ready = true;
                    break;
                }
                semio_framework_plugin::app::ChildEmitPreparationStep::Pending => {}
                semio_framework_plugin::app::ChildEmitPreparationStep::Refused(fault) => panic!("example {:?} child edit refused: {}", example.id(), fault.message),
            }
        }
        assert!(ready, "example {:?} prepares its content edit within its bound", example.id());
    }
}
