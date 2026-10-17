//! ⚖️ Every verb the wires editor declares honours its declaration — the framework's declared-verb law
//! (`artifact_app_laws::assert_declared_verbs_honour_their_declarations`) over this surface, with the
//! example invocations of `🧫️fixtures/⚖️declared-verb-examples.json` on the metabolism boot example.

use semio_framework_plugin::EditorApp;
use crate::editor::wires::*;
use semio_framework_plugin::artifact_app_laws::{assert_declared_verbs_honour_their_declarations, declared_verb_agent_divergences, declared_verb_probe, declared_verb_staged_wrote_document, declared_verb_wrote_document};

#[semio_framework_async_macros::async_test]
async fn every_declared_wires_verb_honours_its_declaration() {
    let examples = runtime_fixture_text("../../🪆️subsets/✳️any/✏️editor/🧪️tests/🧫️fixtures/⚖️declared-verb-examples.json");
    let probes = assert_declared_verbs_honour_their_declarations::<EditorApp<ReasoningWiresPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>(create_wires_app, Some(examples), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
    assert_eq!(probes.len(), 8, "the wires editor declares eight verbs");
    let relate = declared_verb_probe(&probes, "addRelationship");
    assert!(declared_verb_staged_wrote_document(relate), "the example relationship is written: {:?}", relate.windows);
    let canvas = &relate.windows[0];
    assert_eq!(canvas.arguments.iter().map(|argument| argument.argument.as_str()).collect::<Vec<_>>(), ["sourceId", "targetId", "kind"], "every declared argument is perturbed");
    assert!(declared_verb_wrote_document(&canvas.arguments[2].first) && declared_verb_wrote_document(&canvas.arguments[2].second), "both kinds write an edge");
    let findings = probes.iter().map(|probe| format!("{}: {:?}", probe.verb, probe.agent)).collect::<Vec<_>>();
    assert_eq!(declared_verb_agent_divergences(&probes), ["setActiveExample"], "the agent lane previews no operation for a LoadDocument example switch: {findings:#?}");
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
