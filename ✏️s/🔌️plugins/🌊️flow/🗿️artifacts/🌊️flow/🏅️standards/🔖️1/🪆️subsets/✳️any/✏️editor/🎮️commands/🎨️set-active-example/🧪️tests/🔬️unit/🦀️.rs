use super::*;
use crate::editor::flow::unit_tests::context::{dispatch, flow_app};
use crate::editor::flow::FlowCommand;
use crate::examples::demo;
use semio_framework_plugin::artifact_app_laws::{meta, settle_registered_typed_operation};
use semio_framework_plugin::Effect;
use store::ArtifactPack;

fn loaded<'a>(effects: impl Iterator<Item = &'a Effect>) -> Vec<FlowSnapshot> {
    effects.filter_map(|effect| if let Effect::LoadDocument { pack, .. } = effect { Some(<FlowSnapshot as ArtifactPack>::decode_pack(pack).expect("the loaded pack decodes")) } else { None }).collect()
}

/// ⚖️ LAW: `setActiveExample demo` is a load, not an edit: it requests exactly one `LoadDocument` of the published demo graph and publishes no mutation row,
/// so no history version is minted and nothing is differenced against the live scene.
#[semio_framework_async_macros::async_test]
async fn set_active_example_demo_requests_one_load_of_the_published_demo_graph() {
    let mut app = flow_app().await;
    let expected = demo::snapshot_from_text(demo::PRIMARY_TEXT).expect("demo parses");
    let result = dispatch(&mut app, FlowCommand::SetActiveExample(SetActiveExample { example_id: demo::ID.into() })).await;
    let receipt = settle_registered_typed_operation(&mut app, meta("local").instance_id).await.expect("retained Flow command publication");
    let documents = loaded(result.requested_effects.iter().chain(receipt.effects.iter()));
    assert_eq!(documents.len(), 1, "exactly one load-document effect");
    assert_eq!(documents[0].to_host_snapshot().widgets, expected.to_host_snapshot().widgets);
}

/// 📚️ LAW: every example the editor ships loads through its own `setActiveExample` route as one load-document effect and no mutation row.
#[test]
fn every_shipped_example_loads_through_set_active_example() {
    use semio_framework_plugin::ArtifactEditor;
    let examples = crate::editor::flow::FlowPlayApp::examples();
    assert!(!examples.is_empty(), "flow ships at least one example");
    for example in &examples {
        let loaded_example = demo::snapshot_from_text(&example.document_json()).unwrap_or_else(|error| panic!("example {:?} must parse through the route's own boundary: {error}", example.id()));
        loaded_example.to_host_snapshot().retire_cold();
        let emit = set_active_example_edit(&SetActiveExample { example_id: example.id().to_string() }).unwrap_or_else(|fault| panic!("setActiveExample {:?} must load its own shipped asset: {}", example.id(), fault.message));
        assert_eq!(loaded(emit.effects.iter()).len(), 1, "example {:?} requests one load", example.id());
    }
}
